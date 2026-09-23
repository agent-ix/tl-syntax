use std::collections::BTreeMap;
use std::path::PathBuf;

use tl_release_gate::{
    authorized_tag_plan, check, compare_legacy_goldens, consumer_lock_differences,
    consumer_manifest, duplicated_owner_blobs, parse_semver_findings, reconcile_api_migrations,
    test_target_has_executed_cases, ApiFinding, Candidate, CandidateFact, CandidateInput,
    CandidateSet, HistoricalLane, HumanReleaseDecision, CRATES,
};

fn sha(digit: char) -> String {
    digit.to_string().repeat(40)
}

fn selected(name: &str) -> String {
    sha(match name {
        "tl-syntax" => '1',
        "tl-parse" => '2',
        "tl-mltl" => '3',
        "tl-rewrite" => '4',
        _ => panic!("unexpected crate"),
    })
}

fn dependency(name: &str, revision: &str) -> String {
    format!(
        "{name} = {{ version = \"=0.4.0\", git = \"https://github.com/agent-ix/{name}.git\", rev = \"{revision}\" }}\n"
    )
}

fn fixture() -> (CandidateSet, Vec<CandidateInput>) {
    let mut set = CandidateSet {
        candidates: Vec::new(),
        historical_lanes: Vec::new(),
        require_tags: false,
    };
    let mut inputs = Vec::new();
    for name in CRATES {
        let commit = selected(name);
        let tag = format!("{name}-v0.4.0");
        set.candidates.push(Candidate {
            name: name.to_owned(),
            path: PathBuf::from(format!("/tmp/{name}")),
            commit: commit.clone(),
            proposed_tag: Some(tag.clone()),
            previous_tag: "v0.3.0".to_owned(),
        });
        let deps: Vec<_> = match name {
            "tl-syntax" => vec![],
            "tl-parse" | "tl-mltl" => vec!["tl-syntax"],
            "tl-rewrite" => vec!["tl-syntax", "tl-parse", "tl-mltl"],
            _ => unreachable!(),
        };
        let mut manifest = format!(
            "[package]\nname = \"{name}\"\nversion = \"0.4.0\"\nrust-version = \"1.98.1\"\n[dependencies]\n"
        );
        let mut lock = "version = 4\n".to_owned();
        for dep in deps {
            let rev = selected(dep);
            manifest.push_str(&dependency(dep, &rev));
            lock.push_str(&format!(
                "[[package]]\nname = \"{dep}\"\nversion = \"0.4.0\"\nsource = \"git+https://github.com/agent-ix/{dep}.git?rev={rev}#{rev}\"\n"
            ));
        }
        inputs.push(CandidateInput {
            fact: CandidateFact {
                name: name.to_owned(),
                commit: commit.clone(),
                version: "0.4.0".to_owned(),
                msrv: "1.98.1".to_owned(),
                proposed_tag: Some(tag),
                tag_commit: Some(commit),
            },
            manifest: manifest.parse().unwrap(),
            lockfile: lock.parse().unwrap(),
            toolchain: "[toolchain]\nchannel = \"1.98.1\"".parse().unwrap(),
        });
    }
    (set, inputs)
}

fn failure(set: &CandidateSet, inputs: &[CandidateInput], part: &str) {
    let report = check(set, inputs);
    assert!(!report.accepted, "mutation unexpectedly passed");
    assert!(
        report.failures.iter().any(|item| item.contains(part)),
        "missing {part:?} in {:?}",
        report.failures
    );
}

/// TC-170: each exact production edge and lockfile revision is reconciled.
#[test]
fn exact_candidate_graph_and_wrong_pin_refusal() {
    let (set, mut inputs) = fixture();
    let report = check(&set, &inputs);
    assert!(report.accepted, "{:?}", report.failures);
    assert_eq!(report.edges.len(), 5);
    inputs[1].manifest["dependencies"]["tl-syntax"]["rev"] = toml::Value::String(sha('9'));
    failure(&set, &inputs, "tl-parse:tl-syntax: normal edge revision");
    inputs[1].manifest["dependencies"]["tl-syntax"]["rev"] =
        toml::Value::String(selected("tl-syntax"));
    inputs[1].lockfile["package"][0]["source"] = toml::Value::String(format!(
        "git+https://github.com/agent-ix/tl-syntax.git?rev={}#{}",
        sha('9'),
        sha('9')
    ));
    failure(&set, &inputs, "lockfile lacks selected tl-syntax");
    inputs[1].manifest["dependencies"]["tl-syntax"]["version"] =
        toml::Value::String("0.4".to_owned());
    failure(&set, &inputs, "exact version");
    inputs[1].manifest["dependencies"]["tl-syntax"]
        .as_table_mut()
        .unwrap()
        .insert(
            "package".to_owned(),
            toml::Value::String("unrelated".to_owned()),
        );
    failure(&set, &inputs, "target unrelated has no selected candidate");
}

/// TC-171: a wrong MSRV or absent/wrong tag cannot pass as a release graph.
#[test]
fn msrv_and_tag_mutations_fail() {
    let (set, mut inputs) = fixture();
    inputs[2].fact.msrv = "1.99.0".to_owned();
    failure(&set, &inputs, "MSRV 1.99.0 differs");
    inputs[2].fact.msrv = "1.98.1".to_owned();
    let mut set = set;
    set.require_tags = true;
    inputs[0].fact.tag_commit = None;
    failure(&set, &inputs, "proposed tag is absent");
    inputs[0].fact.tag_commit = Some(sha('9'));
    failure(&set, &inputs, "does not target candidate");
}

/// TC-172: only a declared, isolated dev pin can retain an older revision.
#[test]
fn historical_test_lane_is_explicit_and_cannot_enter_production() {
    let (mut set, mut inputs) = fixture();
    let old = sha('a');
    let dev: toml::Value = format!(
        "[dev-dependencies]\nlegacy_syntax = {{ package = \"tl-syntax\", version = \"=0.3.0\", git = \"https://github.com/agent-ix/tl-syntax.git\", rev = \"{old}\" }}\n"
    )
    .parse()
    .unwrap();
    inputs[3].manifest.as_table_mut().unwrap().insert(
        "dev-dependencies".to_owned(),
        dev["dev-dependencies"].clone(),
    );
    inputs[3].lockfile["package"].as_array_mut().unwrap().push(
        format!(
            "name = \"tl-syntax\"\nversion = \"0.3.0\"\nsource = \"git+https://github.com/agent-ix/tl-syntax.git?rev={old}#{old}\""
        )
        .parse()
        .unwrap(),
    );
    failure(&set, &inputs, "dev edge revision");
    set.historical_lanes.push(HistoricalLane {
        owner: "tl-rewrite".to_owned(),
        alias: "legacy_syntax".to_owned(),
        package: "tl-syntax".to_owned(),
        revision: old.clone(),
        version: "0.3.0".to_owned(),
        test_target: "legacy_syntax".to_owned(),
    });
    let report = check(&set, &inputs);
    assert!(report.accepted, "{:?}", report.failures);
    let old_entry = inputs[3].lockfile["package"]
        .as_array_mut()
        .unwrap()
        .pop()
        .unwrap();
    failure(&set, &inputs, "lockfile lacks edge revision");
    inputs[3].lockfile["package"]
        .as_array_mut()
        .unwrap()
        .push(old_entry);
    let table = inputs[3].manifest.as_table_mut().unwrap();
    let dev = table.remove("dev-dependencies").unwrap();
    table.insert("build-dependencies".to_owned(), dev);
    failure(&set, &inputs, "build edge revision");
}

/// TC-176 (syntax owner): old wire bytes cannot be changed or removed.
#[test]
fn syntax_legacy_wire_goldens_are_immutable() {
    let old: BTreeMap<_, _> = [
        (
            "corpus/schema/formula-v1.schema.json".to_owned(),
            b"schema\n".to_vec(),
        ),
        (
            "corpus/formulas/primitive-true.json".to_owned(),
            b"true\n".to_vec(),
        ),
    ]
    .into();
    let mut current = old.clone();
    current.insert(
        "corpus/schema/formula-v3.schema.json".to_owned(),
        b"new\n".to_vec(),
    );
    assert!(compare_legacy_goldens(&old, &current).is_empty());
    current.insert(
        "corpus/formulas/primitive-true.json".to_owned(),
        b"false\n".to_vec(),
    );
    assert!(compare_legacy_goldens(&old, &current)
        .iter()
        .any(|difference| difference.contains("bytes changed")));
    current.remove("corpus/schema/formula-v1.schema.json");
    assert!(compare_legacy_goldens(&old, &current)
        .iter()
        .any(|difference| difference.contains("is missing")));
    assert!(!compare_legacy_goldens(&BTreeMap::new(), &current).is_empty());
}

/// TC-162: moving a copied owner file to another path cannot evade isolation.
#[test]
fn syntax_owner_corpus_is_not_vendored_by_a_consumer() {
    let owner = BTreeMap::from([
        (
            "corpus/infinite-trace/cases.json".to_owned(),
            "owner-cases-blob".to_owned(),
        ),
        (
            "corpus/infinite-trace/schema.json".to_owned(),
            "owner-schema-blob".to_owned(),
        ),
    ]);
    let mut consumer = BTreeMap::from([(
        "corpus/parser/grammar.json".to_owned(),
        "separate-parser-fixture".to_owned(),
    )]);
    assert!(duplicated_owner_blobs(&owner, &consumer).is_empty());
    consumer.insert(
        "fixtures/copied-cases.json".to_owned(),
        "owner-cases-blob".to_owned(),
    );
    assert_eq!(
        duplicated_owner_blobs(&owner, &consumer),
        ["fixtures/copied-cases.json".to_owned()]
    );
}

/// TC-176: every old path in every crate must retain its exact bytes.
#[test]
fn all_crates_legacy_wire_goldens_are_immutable() {
    for name in CRATES {
        let previous = BTreeMap::from([(
            format!("corpus/{name}/wire.json"),
            format!("{name}-v0.3.0").into_bytes(),
        )]);
        let mut current = previous.clone();
        assert!(compare_legacy_goldens(&previous, &current).is_empty());
        current.insert(
            format!("corpus/{name}/wire.json"),
            b"changed legacy wire".to_vec(),
        );
        assert_eq!(compare_legacy_goldens(&previous, &current).len(), 1);
    }
}

/// TC-175: duplicate rustdoc paths collapse, while unlisted breaks and stale
/// migration entries both refuse the exact version section.
#[test]
fn api_findings_require_matching_migration_entries() {
    let output = "--- failure enum_variant_added: new enum variant ---\nFailed in:\n  variant SemanticProfile:InfiniteTraceV1 in /tmp/src/profile.rs:21\n  variant SemanticProfile:InfiniteTraceV1 in /tmp/src/profile.rs:21\n\n--- warning partial_ord_enum_variants_reordered: order changed ---\nFailed in:\n  FormulaError::Old moved from position 7 to 8, in /tmp/src/graph.rs:5\n";
    let findings = parse_semver_findings(output).unwrap();
    assert_eq!(findings.len(), 2);
    let changelog = "## 0.4.0\n### API migration inventory\n- `enum_variant_added` `SemanticProfile:InfiniteTraceV1`: Migration: handle the new profile explicitly.\n- `partial_ord_enum_variants_reordered` `FormulaError::Old`: Migration: choose an explicit sort key.\n\n## 0.3.0\n";
    assert!(reconcile_api_migrations(changelog, "0.4.0", &findings).is_empty());
    let missing = changelog.replace("- `enum_variant_added` `SemanticProfile:InfiniteTraceV1`: Migration: handle the new profile explicitly.\n", "");
    assert!(reconcile_api_migrations(&missing, "0.4.0", &findings)
        .iter()
        .any(|failure| failure.contains("unmapped API finding")));
    let stale = changelog.replace(
        "## 0.3.0",
        "- `enum_variant_added` `Extra::Variant`: Migration: handle it.\n## 0.3.0",
    );
    assert!(reconcile_api_migrations(&stale, "0.4.0", &findings)
        .iter()
        .any(|failure| failure.contains("stale API migration entry")));
    let empty = changelog.replace(
        "Migration: handle the new profile explicitly.",
        "Migration: ",
    );
    assert!(reconcile_api_migrations(&empty, "0.4.0", &findings)
        .iter()
        .any(|failure| failure.contains("empty migration")));
    assert!(parse_semver_findings("Failed in:\n  variant X:Y in /tmp/x.rs:1").is_err());
}

/// TC-175: the measured syntax 0.3-to-0.4 API findings have migration text.
#[test]
fn syntax_measured_api_breaks_are_documented() {
    let findings = [
        ApiFinding {
            lint: "enum_variant_added".to_owned(),
            symbol: "SemanticProfile:InfiniteTraceV1".to_owned(),
        },
        ApiFinding {
            lint: "enum_no_repr_variant_discriminant_changed".to_owned(),
            symbol: "FormulaError::FormulaV1NodeUnsupported".to_owned(),
        },
        ApiFinding {
            lint: "partial_ord_enum_variants_reordered".to_owned(),
            symbol: "FormulaError::FormulaV1NodeUnsupported".to_owned(),
        },
    ];
    assert!(
        reconcile_api_migrations(include_str!("../../CHANGELOG.md"), "0.4.0", &findings).is_empty()
    );
}

/// TC-177/178: the consumer cannot silently resolve a path or older pin.
#[test]
fn external_consumer_manifest_and_lock_bind_all_four_commits() {
    let (_, inputs) = fixture();
    let facts: Vec<_> = inputs.iter().map(|input| input.fact.clone()).collect();
    let manifest = consumer_manifest(&facts).unwrap();
    let parsed: toml::Value = manifest.parse().unwrap();
    assert_eq!(parsed["package"]["name"].as_str(), Some("tl-release-smoke"));
    assert!(parsed.get("workspace").is_none());
    for name in CRATES {
        let entry = &parsed["dependencies"][name];
        assert_eq!(entry["rev"].as_str(), Some(selected(name).as_str()));
        assert!(entry.get("path").is_none());
        assert_eq!(
            entry["git"].as_str(),
            Some(format!("https://github.com/agent-ix/{name}.git").as_str())
        );
    }
    let mut lock = "version = 4\n".to_owned();
    for name in CRATES {
        let revision = selected(name);
        lock.push_str(&format!(
            "[[package]]\nname = \"{name}\"\nversion = \"0.4.0\"\nsource = \"git+https://github.com/agent-ix/{name}.git?rev={revision}#{revision}\"\n"
        ));
    }
    let mut lock: toml::Value = lock.parse().unwrap();
    assert!(consumer_lock_differences(&lock, &facts).is_empty());
    lock["package"][0]["source"] = toml::Value::String("path+../tl-syntax".to_owned());
    assert_eq!(consumer_lock_differences(&lock, &facts).len(), 1);
    lock["package"].as_array_mut().unwrap().remove(1);
    assert_eq!(consumer_lock_differences(&lock, &facts).len(), 2);
}

/// TC-179: no tag plan escapes without a current human decision on this graph.
#[test]
fn exact_human_decision_only_yields_dependency_ordered_tag_plan() {
    let (_, inputs) = fixture();
    let mut facts: Vec<_> = inputs.iter().map(|input| input.fact.clone()).collect();
    for fact in &mut facts {
        fact.tag_commit = None;
    }
    let now = 1_000_u64;
    let mut decision = HumanReleaseDecision {
        decision_id: "FR-018-reviewed".to_owned(),
        human_actor: "release-owner".to_owned(),
        accepted: true,
        expires_unix_seconds: now + 100,
        candidate_revisions: facts
            .iter()
            .map(|fact| (fact.name.clone(), fact.commit.clone()))
            .collect(),
        evidence_sha256: "a".repeat(64),
    };
    assert!(authorized_tag_plan(&facts, None, true, now).is_err());
    assert!(authorized_tag_plan(&facts, Some(&decision), false, now).is_err());
    let plan = authorized_tag_plan(&facts, Some(&decision), true, now).unwrap();
    assert_eq!(
        plan.iter()
            .map(|action| action.name.as_str())
            .collect::<Vec<_>>(),
        CRATES
    );
    decision
        .candidate_revisions
        .insert("tl-parse".to_owned(), sha('9'));
    assert!(authorized_tag_plan(&facts, Some(&decision), true, now).is_err());
    decision
        .candidate_revisions
        .insert("tl-parse".to_owned(), selected("tl-parse"));
    decision.expires_unix_seconds = now;
    assert!(authorized_tag_plan(&facts, Some(&decision), true, now).is_err());
    decision.expires_unix_seconds = now + 100;
    facts[0].tag_commit = Some(facts[0].commit.clone());
    assert!(authorized_tag_plan(&facts, Some(&decision), true, now).is_err());
}

/// TC-174: Cargo's success exit is insufficient when every test was ignored.
#[test]
fn corpus_target_cannot_pass_with_an_empty_or_ignored_population() {
    assert!(test_target_has_executed_cases(
        "running 1 test\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
    ));
    assert!(!test_target_has_executed_cases(
        "running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
    ));
    assert!(!test_target_has_executed_cases(
        "running 1 test\ntest result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out"
    ));
}
