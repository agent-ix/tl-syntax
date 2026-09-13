#![cfg(feature = "serde")]

use serde::Deserialize;

const MANIFEST: &str = include_str!("../spec/past-profile-implementation.json");
const FORMAT: &str = "tl-syntax.past-profile-implementation/v1";
const EPIC: &str = "agent-ix/tl-syntax#52";
const MRS_003: &str = "568a5f18ea496232e0fa9eff7506990bfcecfefa";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    format: String,
    owner_epic: String,
    authorization: Authorization,
    prerequisites: Vec<Prerequisite>,
    tasks: Vec<Task>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Authorization {
    state: String,
    revision: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct Prerequisite {
    repository: String,
    kind: String,
    revision: String,
    state: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct Task {
    repository: String,
    issue: u64,
    owner: String,
    predecessors: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ManifestRefusal {
    field: &'static str,
    task: Option<String>,
}

impl ManifestRefusal {
    const fn code(&self) -> &'static str {
        "past_profile_dependency_manifest_invalid"
    }
}

fn refusal(field: &'static str) -> ManifestRefusal {
    ManifestRefusal { field, task: None }
}

fn task_refusal(field: &'static str, task: &Task) -> ManifestRefusal {
    ManifestRefusal {
        field,
        task: Some(format!("{}#{}", task.repository, task.issue)),
    }
}

fn expected_prerequisites() -> [Prerequisite; 3] {
    [
        Prerequisite {
            repository: "agent-ix/tl-syntax".into(),
            kind: "m0".into(),
            revision: "26b801d6567645b637be20bef5c256d0ea4ed45c".into(),
            state: "accepted".into(),
        },
        Prerequisite {
            repository: "agent-ix/tl-syntax".into(),
            kind: "mrs-002".into(),
            revision: "8d3ff9873acdef9b0af03f05971620c07eacf733".into(),
            state: "accepted".into(),
        },
        Prerequisite {
            repository: "agent-ix/tl-syntax".into(),
            kind: "mrs-003".into(),
            revision: MRS_003.into(),
            state: "accepted".into(),
        },
    ]
}

fn expected_tasks() -> [Task; 7] {
    [
        Task {
            repository: "agent-ix/tl-syntax".into(),
            issue: 53,
            owner: "tl-syntax".into(),
            predecessors: vec![],
        },
        Task {
            repository: "agent-ix/tl-parse".into(),
            issue: 35,
            owner: "tl-parse".into(),
            predecessors: vec!["agent-ix/tl-syntax#53".into()],
        },
        Task {
            repository: "agent-ix/tl-mltl".into(),
            issue: 63,
            owner: "tl-mltl".into(),
            predecessors: vec!["agent-ix/tl-syntax#53".into()],
        },
        Task {
            repository: "agent-ix/tl-rewrite".into(),
            issue: 38,
            owner: "tl-rewrite".into(),
            predecessors: vec!["agent-ix/tl-syntax#53".into()],
        },
        Task {
            repository: "agent-ix/tl-syntax".into(),
            issue: 54,
            owner: "tl-syntax".into(),
            predecessors: vec![
                "agent-ix/tl-mltl#63".into(),
                "agent-ix/tl-parse#35".into(),
                "agent-ix/tl-rewrite#38".into(),
            ],
        },
        Task {
            repository: "agent-ix/quire-contract-ir".into(),
            issue: 70,
            owner: "quire-contract-ir".into(),
            predecessors: vec!["agent-ix/quire-contract-ir#63".into()],
        },
        Task {
            repository: "agent-ix/quire-contract-ir".into(),
            issue: 71,
            owner: "quire-contract-ir".into(),
            predecessors: vec![
                "agent-ix/quire-contract-ir#64".into(),
                "agent-ix/quire-contract-ir#70".into(),
                "agent-ix/tl-syntax#54".into(),
            ],
        },
    ]
}

fn validate(manifest: &Manifest) -> Result<(), ManifestRefusal> {
    if manifest.format != FORMAT {
        return Err(refusal("format"));
    }
    if manifest.owner_epic != EPIC {
        return Err(refusal("owner_epic"));
    }
    if manifest.authorization.state != "authorized" {
        return Err(refusal("authorization.state"));
    }
    if manifest.authorization.revision.as_deref() != Some(MRS_003) {
        return Err(refusal("authorization.revision"));
    }
    let expected_prerequisites = expected_prerequisites();
    if manifest.prerequisites.len() != expected_prerequisites.len() {
        return Err(refusal("prerequisites"));
    }
    let prerequisite_fields = [
        [
            "prerequisites[0].repository",
            "prerequisites[0].kind",
            "prerequisites[0].revision",
            "prerequisites[0].state",
        ],
        [
            "prerequisites[1].repository",
            "prerequisites[1].kind",
            "prerequisites[1].revision",
            "prerequisites[1].state",
        ],
        [
            "prerequisites[2].repository",
            "prerequisites[2].kind",
            "prerequisites[2].revision",
            "prerequisites[2].state",
        ],
    ];
    for ((actual, expected), fields) in manifest
        .prerequisites
        .iter()
        .zip(expected_prerequisites)
        .zip(prerequisite_fields)
    {
        if actual.repository != expected.repository {
            return Err(refusal(fields[0]));
        }
        if actual.kind != expected.kind {
            return Err(refusal(fields[1]));
        }
        if actual.revision != expected.revision {
            return Err(refusal(fields[2]));
        }
        if actual.state != expected.state {
            return Err(refusal(fields[3]));
        }
    }
    let expected = expected_tasks();
    if manifest.tasks.len() != expected.len() {
        return Err(refusal("tasks"));
    }
    for (task, expected) in manifest.tasks.iter().zip(expected) {
        if task.repository != expected.repository || task.issue != expected.issue {
            return Err(task_refusal("identity", task));
        }
        if task.owner != expected.owner {
            return Err(task_refusal("owner", task));
        }
        if task.predecessors != expected.predecessors {
            return Err(task_refusal("predecessors", task));
        }
        if task.predecessors.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(task_refusal("predecessor_order", task));
        }
    }
    Ok(())
}

fn parsed_manifest() -> Manifest {
    serde_json::from_str(MANIFEST).unwrap()
}

// Trace: TC-058, FR-013-AC-5
#[test]
fn accepted_dependency_manifest_authorizes_the_exact_implementation_dag() {
    validate(&parsed_manifest()).unwrap();
}

// Trace: TC-058, FR-013-AC-5
#[test]
fn every_dependency_manifest_axis_fails_closed_with_a_stable_refusal() {
    fn rejected(manifest: Manifest, field: &'static str) {
        let rejected = validate(&manifest).unwrap_err();
        assert_eq!(rejected.code(), "past_profile_dependency_manifest_invalid");
        assert_eq!(rejected.field, field);
    }

    let mut manifest = parsed_manifest();
    manifest.format.push_str("-unknown");
    rejected(manifest, "format");

    let mut manifest = parsed_manifest();
    manifest.owner_epic = "agent-ix/tl-syntax#0".into();
    rejected(manifest, "owner_epic");

    let mut manifest = parsed_manifest();
    manifest.authorization.state = "blocked".into();
    rejected(manifest, "authorization.state");

    let mut manifest = parsed_manifest();
    manifest.authorization.revision = None;
    rejected(manifest, "authorization.revision");

    for index in 0..3 {
        let revision_fields = [
            "prerequisites[0].revision",
            "prerequisites[1].revision",
            "prerequisites[2].revision",
        ];
        let mut manifest = parsed_manifest();
        manifest.prerequisites[index].revision = "unaccepted".into();
        rejected(manifest, revision_fields[index]);

        let state_fields = [
            "prerequisites[0].state",
            "prerequisites[1].state",
            "prerequisites[2].state",
        ];
        let mut manifest = parsed_manifest();
        manifest.prerequisites[index].state = "pending".into();
        rejected(manifest, state_fields[index]);
    }

    let mut manifest = parsed_manifest();
    manifest.tasks[0].issue = 0;
    rejected(manifest, "identity");

    let mut manifest = parsed_manifest();
    manifest.tasks[1].owner = "tl-syntax".into();
    rejected(manifest, "owner");

    let mut manifest = parsed_manifest();
    manifest.tasks[6].predecessors.pop().unwrap();
    rejected(manifest, "predecessors");

    let mut manifest = parsed_manifest();
    manifest.tasks.pop().unwrap();
    rejected(manifest, "tasks");
}
