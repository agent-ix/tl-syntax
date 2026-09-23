//! Exact candidate graph admission for a four-crate TL release.
//!
//! All manifest and lockfile facts come from the specified Git commits. The
//! executable additionally checks tag targets and builds at the shared MSRV.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const CRATES: [&str; 4] = ["tl-syntax", "tl-parse", "tl-mltl", "tl-rewrite"];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub name: String,
    pub path: PathBuf,
    pub commit: String,
    #[serde(default)]
    pub proposed_tag: Option<String>,
    pub previous_tag: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalLane {
    pub owner: String,
    pub alias: String,
    pub package: String,
    pub revision: String,
    pub version: String,
    /// The owner test target run with `cargo test --locked --test <test_target>`.
    pub test_target: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateSet {
    pub candidates: Vec<Candidate>,
    #[serde(default)]
    pub historical_lanes: Vec<HistoricalLane>,
    /// Enable only when checking an already created proposed tag.
    #[serde(default)]
    pub require_tags: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct CandidateFact {
    pub name: String,
    pub commit: String,
    pub version: String,
    pub msrv: String,
    pub proposed_tag: Option<String>,
    pub tag_commit: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct EdgeFact {
    pub owner: String,
    pub alias: String,
    pub package: String,
    pub scope: String,
    pub version: Option<String>,
    pub revision: Option<String>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct GateReport {
    pub schema: &'static str,
    pub candidates: Vec<CandidateFact>,
    pub edges: Vec<EdgeFact>,
    pub failures: Vec<String>,
    /// Candidate builds executed by the CLI after the static graph passes.
    pub msrv_builds: Vec<BuildFact>,
    /// Byte-level result for the syntax owner's retained legacy golden files.
    pub syntax_wire_golden: Option<WireFact>,
    pub accepted: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct WireFact {
    pub previous_tag: String,
    pub checked_files: usize,
    pub differences: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BuildFact {
    pub name: String,
    pub result: String,
}

#[derive(Clone, Debug)]
pub struct CandidateInput {
    pub fact: CandidateFact,
    pub manifest: toml::Value,
    pub lockfile: toml::Value,
    pub toolchain: toml::Value,
}

fn tl_package(alias: &str, value: &toml::Value) -> Option<String> {
    let name = value
        .as_table()
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .unwrap_or(alias);
    let source_names_tl = value
        .as_table()
        .and_then(|table| table.get("git"))
        .and_then(toml::Value::as_str)
        .is_some_and(|url| {
            CRATES
                .iter()
                .any(|crate_name| url == format!("https://github.com/agent-ix/{crate_name}.git"))
        });
    (CRATES.contains(&name) || CRATES.contains(&alias) || source_names_tl).then(|| name.to_owned())
}

fn read_edges(owner: &str, table: &toml::map::Map<String, toml::Value>, edges: &mut Vec<EdgeFact>) {
    for (key, scope) in [
        ("dependencies", "normal"),
        ("build-dependencies", "build"),
        ("dev-dependencies", "dev"),
    ] {
        if let Some(deps) = table.get(key).and_then(toml::Value::as_table) {
            for (alias, value) in deps {
                let Some(package) = tl_package(alias, value) else {
                    continue;
                };
                let attrs = value.as_table();
                let string = |key| {
                    attrs
                        .and_then(|table| table.get(key))
                        .and_then(toml::Value::as_str)
                        .map(str::to_owned)
                };
                edges.push(EdgeFact {
                    owner: owner.to_owned(),
                    alias: alias.to_owned(),
                    package,
                    scope: scope.to_owned(),
                    version: string("version").or_else(|| value.as_str().map(str::to_owned)),
                    revision: string("rev"),
                    source: string("git"),
                });
            }
        }
    }
    if let Some(targets) = table.get("target").and_then(toml::Value::as_table) {
        for (_, target) in targets {
            if let Some(target) = target.as_table() {
                read_edges(owner, target, edges);
            }
        }
    }
}

fn package_field<'a>(value: &'a toml::Value, key: &str) -> Option<&'a str> {
    value.get("package")?.as_table()?.get(key)?.as_str()
}

#[derive(Clone, Debug)]
struct LockedEntry {
    version: String,
    source: String,
    revision: String,
}

fn locked_entries(lock: &toml::Value, name: &str) -> Vec<LockedEntry> {
    lock.get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|entry| entry.get("name").and_then(toml::Value::as_str) == Some(name))
        .filter_map(|entry| {
            let version = entry.get("version")?.as_str()?.to_owned();
            let source = entry.get("source")?.as_str()?.to_owned();
            let revision = source.rsplit_once('#')?.1.to_owned();
            Some(LockedEntry {
                version,
                source,
                revision,
            })
        })
        .collect()
}

/// Validates the supplied immutable Git facts, all four manifests and lockfiles.
/// It collects every inconsistent edge rather than stopping at the first one.
pub fn check(set: &CandidateSet, inputs: &[CandidateInput]) -> GateReport {
    let mut failures = Vec::new();
    let mut selected = BTreeMap::new();
    let mut facts = Vec::new();
    let mut edges = Vec::new();
    for name in CRATES {
        let matches: Vec<_> = inputs
            .iter()
            .filter(|input| input.fact.name == name)
            .collect();
        if matches.len() != 1 {
            failures.push(format!(
                "{name}: expected exactly one candidate, found {}",
                matches.len()
            ));
            continue;
        }
        let input = matches[0];
        let fact = &input.fact;
        let declarations: Vec<_> = set
            .candidates
            .iter()
            .filter(|candidate| candidate.name == name)
            .collect();
        if declarations.len() != 1
            || declarations[0].commit != fact.commit
            || declarations[0].proposed_tag != fact.proposed_tag
        {
            failures.push(format!(
                "{name}: observed candidate differs from declaration"
            ));
        }
        if !valid_sha(&fact.commit) {
            failures.push(format!("{name}: candidate commit is not a full SHA-1"));
        }
        if set.require_tags && fact.tag_commit.as_deref() != Some(fact.commit.as_str()) {
            failures.push(format!(
                "{name}: proposed tag is absent or does not target candidate {}",
                fact.commit
            ));
        }
        if package_field(&input.manifest, "name") != Some(name)
            || package_field(&input.manifest, "version") != Some(fact.version.as_str())
            || package_field(&input.manifest, "rust-version") != Some(fact.msrv.as_str())
        {
            failures.push(format!(
                "{name}: candidate package identity/version/MSRV differs from its manifest"
            ));
        }
        let channel = input
            .toolchain
            .get("toolchain")
            .and_then(|value| value.get("channel"))
            .and_then(toml::Value::as_str);
        if channel != Some(fact.msrv.as_str()) {
            failures.push(format!(
                "{name}: rust-toolchain channel disagrees with MSRV {}",
                fact.msrv
            ));
        }
        selected.insert(name.to_owned(), fact.clone());
        facts.push(fact.clone());
        if let Some(table) = input.manifest.as_table() {
            read_edges(name, table, &mut edges);
        } else {
            failures.push(format!("{name}: Cargo.toml is not a table"));
        }
    }
    if set.candidates.len() != CRATES.len() || inputs.len() != CRATES.len() {
        failures.push("candidate set must contain exactly the four TL crates".to_owned());
    }
    if let Some(first) = facts.first() {
        for fact in facts.iter().skip(1) {
            if fact.msrv != first.msrv {
                failures.push(format!(
                    "{}: MSRV {} differs from {}",
                    fact.name, fact.msrv, first.msrv
                ));
            }
        }
    }
    let declared_lanes: BTreeSet<_> = set
        .historical_lanes
        .iter()
        .map(|lane| (&lane.owner, &lane.alias, &lane.package, &lane.revision))
        .collect();
    let mut used_lanes = BTreeSet::new();
    for edge in &edges {
        let Some(target) = selected.get(&edge.package) else {
            failures.push(format!(
                "{}:{}: target {} has no selected candidate",
                edge.owner, edge.alias, edge.package
            ));
            continue;
        };
        let expected_source = format!("https://github.com/agent-ix/{}.git", edge.package);
        if edge.source.as_deref() != Some(expected_source.as_str()) {
            failures.push(format!(
                "{}:{}: TL edge must use exact canonical Git source",
                edge.owner, edge.alias
            ));
        }
        let mut historical = None;
        if edge.revision.as_deref() != Some(target.commit.as_str()) {
            let lane = (
                &edge.owner,
                &edge.alias,
                &edge.package,
                edge.revision.as_ref().unwrap_or(&target.commit),
            );
            if edge.scope == "dev" && declared_lanes.contains(&lane) {
                used_lanes.insert(lane);
                historical = set.historical_lanes.iter().find(|candidate| {
                    candidate.owner == edge.owner
                        && candidate.alias == edge.alias
                        && candidate.package == edge.package
                        && edge.revision.as_deref() == Some(candidate.revision.as_str())
                });
            } else {
                failures.push(format!(
                    "{}:{}: {} edge revision {:?} differs from selected {}",
                    edge.owner, edge.alias, edge.scope, edge.revision, target.commit
                ));
            }
        }
        let expected_version =
            historical.map_or(target.version.as_str(), |lane| lane.version.as_str());
        if edge.version.as_deref() != Some(format!("={expected_version}").as_str()) {
            failures.push(format!(
                "{}:{}: exact version does not match selected {} {}",
                edge.owner, edge.alias, edge.package, expected_version
            ));
        }
    }
    for lane in &set.historical_lanes {
        if lane.test_target.is_empty() {
            failures.push(format!(
                "{}:{}: historical lane has no test target",
                lane.owner, lane.alias
            ));
        }
        if !used_lanes.contains(&(&lane.owner, &lane.alias, &lane.package, &lane.revision)) {
            failures.push(format!(
                "{}:{}: declared historical lane is absent or leaks into production",
                lane.owner, lane.alias
            ));
        }
    }
    for input in inputs {
        for edge in edges.iter().filter(|edge| edge.owner == input.fact.name) {
            let resolved = locked_entries(&input.lockfile, &edge.package);
            if !resolved
                .iter()
                .any(|entry| edge.revision.as_deref() == Some(entry.revision.as_str()))
            {
                failures.push(format!(
                    "{}:{}: lockfile lacks edge revision {:?}",
                    edge.owner, edge.alias, edge.revision
                ));
            }
            if edge.scope != "dev" {
                if let Some(target) = selected.get(&edge.package) {
                    if !resolved.iter().any(|entry| entry.revision == target.commit) {
                        failures.push(format!(
                            "{}:{}: lockfile lacks selected {} {}",
                            edge.owner, edge.alias, edge.package, target.commit
                        ));
                    }
                }
            }
        }
        for name in CRATES {
            if name == input.fact.name {
                continue;
            }
            let actual = locked_entries(&input.lockfile, name);
            let expected = selected.get(name);
            for entry in actual {
                let revision = entry.revision;
                let historical = set.historical_lanes.iter().any(|lane| {
                    lane.owner == input.fact.name
                        && lane.package == name
                        && lane.revision == revision
                });
                let expected_version = if historical {
                    set.historical_lanes
                        .iter()
                        .find(|lane| {
                            lane.owner == input.fact.name
                                && lane.package == name
                                && lane.revision == revision
                        })
                        .map(|lane| lane.version.as_str())
                } else {
                    expected.map(|fact| fact.version.as_str())
                };
                if expected_version.is_some_and(|version| entry.version != version) {
                    failures.push(format!(
                        "{}: lockfile resolves {name} at version {} instead of expected {}",
                        input.fact.name,
                        entry.version,
                        expected_version.unwrap_or("absent")
                    ));
                }
                let canonical =
                    format!("git+https://github.com/agent-ix/{name}.git?rev={revision}#{revision}");
                if entry.source != canonical {
                    failures.push(format!(
                        "{}: lockfile resolves {name} from noncanonical source {}",
                        input.fact.name, entry.source
                    ));
                }
                if expected.is_some_and(|fact| revision != fact.commit) && !historical {
                    failures.push(format!(
                        "{}: lockfile resolves {name} at undeclared revision {revision}",
                        input.fact.name
                    ));
                }
            }
        }
    }
    failures.sort();
    failures.dedup();
    GateReport {
        schema: "tl-release-gate/graph-v1",
        candidates: facts,
        edges,
        msrv_builds: Vec::new(),
        syntax_wire_golden: None,
        accepted: failures.is_empty(),
        failures,
    }
}

pub fn valid_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Every old golden path must still exist with identical bytes. New paths are
/// allowed because a new schema edition has its own distinct identity.
pub fn compare_legacy_goldens(
    previous: &BTreeMap<String, Vec<u8>>,
    candidate: &BTreeMap<String, Vec<u8>>,
) -> Vec<String> {
    let mut differences = Vec::new();
    if previous.is_empty() {
        differences.push("previous release has no legacy golden files".to_owned());
    }
    for (path, old) in previous {
        match candidate.get(path) {
            None => differences.push(format!("{path}: legacy golden is missing")),
            Some(new) if new != old => {
                differences.push(format!("{path}: legacy golden bytes changed"))
            }
            Some(_) => {}
        }
    }
    differences
}
