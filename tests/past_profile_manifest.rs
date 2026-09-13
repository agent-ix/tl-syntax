#![cfg(feature = "serde")]

use serde_json::{json, Value};
use tl_syntax::{
    validate_past_profile_implementation, PAST_PROFILE_IMPLEMENTATION_V1,
    PAST_PROFILE_MANIFEST_MAX_BYTES,
};

const MANIFEST: &[u8] = include_bytes!("../spec/past-profile-implementation.json");

fn parsed_manifest() -> Value {
    serde_json::from_slice(MANIFEST).unwrap()
}

fn rejected(value: &Value, field: &'static str) {
    let error =
        validate_past_profile_implementation(&serde_json::to_vec(value).unwrap()).unwrap_err();
    assert_eq!(error.code(), "past_profile_dependency_manifest_invalid");
    assert_eq!(error.field(), field);
}

// Trace: TC-058, FR-013-AC-5
#[test]
fn accepted_dependency_manifest_authorizes_the_exact_implementation_dag() {
    validate_past_profile_implementation(MANIFEST).unwrap();
    assert_eq!(parsed_manifest()["format"], PAST_PROFILE_IMPLEMENTATION_V1);
}

// Trace: TC-058, FR-013-AC-5
#[test]
fn dependency_manifest_wire_and_resource_bounds_fail_closed() {
    for malformed in [b"{".as_slice(), b"[]", b"null"] {
        assert_eq!(
            validate_past_profile_implementation(malformed)
                .unwrap_err()
                .field(),
            "document"
        );
    }
    let mut unknown = parsed_manifest();
    unknown["unknown"] = json!(true);
    rejected(&unknown, "document");
    let oversized = vec![b' '; PAST_PROFILE_MANIFEST_MAX_BYTES + 1];
    assert_eq!(
        validate_past_profile_implementation(&oversized)
            .unwrap_err()
            .field(),
        "size"
    );
}

// Trace: TC-058, FR-013-AC-5
#[test]
fn every_dependency_manifest_axis_fails_closed_with_a_stable_refusal() {
    for (pointer, replacement, field) in [
        ("/format", json!("unknown"), "format"),
        ("/owner_epic", json!("agent-ix/tl-syntax#0"), "owner_epic"),
        (
            "/authorization/state",
            json!("blocked"),
            "authorization.state",
        ),
        (
            "/authorization/revision",
            Value::Null,
            "authorization.revision",
        ),
    ] {
        let mut value = parsed_manifest();
        *value.pointer_mut(pointer).unwrap() = replacement;
        rejected(&value, field);
    }

    let expected_fields = [
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
    for (index, fields) in expected_fields.iter().enumerate() {
        for (field, expected) in ["repository", "kind", "revision", "state"]
            .into_iter()
            .zip(fields)
        {
            let mut value = parsed_manifest();
            *value
                .pointer_mut(&format!("/prerequisites/{index}/{field}"))
                .unwrap() = json!("mutated");
            rejected(&value, expected);
        }
    }

    for index in 0..7 {
        for (field, expected) in [
            ("repository", "identity"),
            ("issue", "identity"),
            ("owner", "owner"),
            ("predecessors", "predecessors"),
        ] {
            let mut value = parsed_manifest();
            let pointer = format!("/tasks/{index}/{field}");
            *value.pointer_mut(&pointer).unwrap() = match field {
                "issue" => json!(999),
                "predecessors" => json!(["mutated#1"]),
                _ => json!("mutated"),
            };
            let error = validate_past_profile_implementation(&serde_json::to_vec(&value).unwrap())
                .unwrap_err();
            assert_eq!(error.field(), expected, "task {index} field {field}");
            assert!(error.task().is_some(), "task {index} field {field}");
        }
    }

    for (pointer, field) in [("/prerequisites", "prerequisites"), ("/tasks", "tasks")] {
        let mut value = parsed_manifest();
        value
            .pointer_mut(pointer)
            .unwrap()
            .as_array_mut()
            .unwrap()
            .pop();
        rejected(&value, field);
    }
    let mut reordered = parsed_manifest();
    reordered["tasks"].as_array_mut().unwrap().swap(0, 1);
    assert_eq!(
        validate_past_profile_implementation(&serde_json::to_vec(&reordered).unwrap())
            .unwrap_err()
            .field(),
        "identity"
    );

    let mut reordered_edges = parsed_manifest();
    reordered_edges["tasks"][4]["predecessors"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    assert_eq!(
        validate_past_profile_implementation(&serde_json::to_vec(&reordered_edges).unwrap())
            .unwrap_err()
            .field(),
        "predecessor_order"
    );
}
