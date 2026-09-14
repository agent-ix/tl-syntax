use alloc::{format, string::String, vec, vec::Vec};
use core::fmt;

use serde::Deserialize;

use super::limits::OWNER_MANIFEST_BYTES;

/// Closed identity of the authorized past-profile implementation manifest.
pub const PAST_PROFILE_IMPLEMENTATION_V1: &str = "tl-syntax.past-profile-implementation/v1";

/// Maximum accepted manifest size before parsing.
pub const PAST_PROFILE_MANIFEST_MAX_BYTES: usize = OWNER_MANIFEST_BYTES;

const OWNER_EPIC: &str = "agent-ix/tl-syntax#52";
const M0: &str = "26b801d6567645b637be20bef5c256d0ea4ed45c";
const MRS_002: &str = "8d3ff9873acdef9b0af03f05971620c07eacf733";
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

/// Stable fail-closed refusal returned by the dependency-manifest gate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PastProfileManifestError {
    field: &'static str,
    task: Option<String>,
}

impl PastProfileManifestError {
    /// Stable machine-readable refusal code.
    pub const fn code(&self) -> &'static str {
        "past_profile_dependency_manifest_invalid"
    }
    /// Manifest field or structural axis that failed validation.
    pub const fn field(&self) -> &'static str {
        self.field
    }
    /// Task identity when the refusal belongs to one DAG node.
    pub fn task(&self) -> Option<&str> {
        self.task.as_deref()
    }
}

impl fmt::Display for PastProfileManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.field)?;
        if let Some(task) = &self.task {
            write!(formatter, " ({task})")?;
        }
        Ok(())
    }
}

fn refusal(field: &'static str) -> PastProfileManifestError {
    PastProfileManifestError { field, task: None }
}

fn task_refusal(field: &'static str, task: &Task) -> PastProfileManifestError {
    PastProfileManifestError {
        field,
        task: Some(format!("{}#{}", task.repository, task.issue)),
    }
}

fn expected_prerequisites() -> [Prerequisite; 3] {
    [
        Prerequisite {
            repository: "agent-ix/tl-syntax".into(),
            kind: "m0".into(),
            revision: M0.into(),
            state: "accepted".into(),
        },
        Prerequisite {
            repository: "agent-ix/tl-syntax".into(),
            kind: "mrs-002".into(),
            revision: MRS_002.into(),
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

/// Parses and validates the exact authorized seven-task implementation DAG.
pub fn validate_past_profile_implementation(bytes: &[u8]) -> Result<(), PastProfileManifestError> {
    if bytes.len() > PAST_PROFILE_MANIFEST_MAX_BYTES {
        return Err(refusal("size"));
    }
    let manifest: Manifest = serde_json::from_slice(bytes).map_err(|_| refusal("document"))?;
    validate(&manifest)
}

fn validate(manifest: &Manifest) -> Result<(), PastProfileManifestError> {
    if manifest.format != PAST_PROFILE_IMPLEMENTATION_V1 {
        return Err(refusal("format"));
    }
    if manifest.owner_epic != OWNER_EPIC {
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
    let fields = [
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
    for ((actual, expected), names) in manifest
        .prerequisites
        .iter()
        .zip(expected_prerequisites)
        .zip(fields)
    {
        if actual.repository != expected.repository {
            return Err(refusal(names[0]));
        }
        if actual.kind != expected.kind {
            return Err(refusal(names[1]));
        }
        if actual.revision != expected.revision {
            return Err(refusal(names[2]));
        }
        if actual.state != expected.state {
            return Err(refusal(names[3]));
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
        if task.predecessors.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(task_refusal("predecessor_order", task));
        }
        if task.predecessors != expected.predecessors {
            return Err(task_refusal("predecessors", task));
        }
    }
    Ok(())
}
