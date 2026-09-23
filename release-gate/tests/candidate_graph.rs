use std::collections::BTreeMap;
use std::path::PathBuf;

use tl_release_gate::{
    check, compare_legacy_goldens, Candidate, CandidateFact, CandidateInput, CandidateSet,
    HistoricalLane, CRATES,
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
