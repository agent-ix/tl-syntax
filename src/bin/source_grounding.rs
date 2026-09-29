//! Project authorial change-assurance inputs through the released Quire source export.
//!
//! This adapter owns only tl-syntax's declaration-to-record mapping. Quire owns
//! specification parsing and source locators; Quoin owns the sealed record.

use std::{
    collections::BTreeSet,
    env, fs,
    path::{Component, Path, PathBuf},
    process,
};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn fail(message: impl Into<String>) -> String {
    message.into()
}

fn member<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value.get(key).ok_or_else(|| fail(format!("missing {key}")))
}

fn string<'a>(value: &'a Value, name: &str) -> Result<&'a str, String> {
    value
        .as_str()
        .ok_or_else(|| fail(format!("{name} must be a string")))
}

fn array<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>, String> {
    value
        .as_array()
        .ok_or_else(|| fail(format!("{name} must be an array")))
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read_json(path: &Path) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn source_bytes(root: &Path, identity: &str) -> Result<Vec<u8>, String> {
    let relative = Path::new(identity);
    if relative.as_os_str().is_empty()
        || !relative
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(fail(format!(
            "source identity is not a relative path: {identity}"
        )));
    }
    let mut current = root.to_path_buf();
    for part in relative.components() {
        current.push(part);
        let metadata = fs::symlink_metadata(&current)
            .map_err(|error| format!("{}: {error}", current.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(fail(format!(
                "source identity crosses a symlink: {identity}"
            )));
        }
    }
    let metadata =
        fs::metadata(&current).map_err(|error| format!("{}: {error}", current.display()))?;
    if !metadata.is_file() {
        return Err(fail(format!(
            "source identity is not a regular file: {identity}"
        )));
    }
    fs::read(&current).map_err(|error| format!("{}: {error}", current.display()))
}

fn check_export(
    export: &Value,
    premises: &Value,
    repository: &str,
    revision: &str,
) -> Result<(), String> {
    if member(export, "format")? != "quire-assurance" || member(export, "format_version")? != 1 {
        return Err(fail("expected the released Quire assurance-v1 export"));
    }
    let source = member(export, "source")?;
    if member(source, "repository")? != repository || member(source, "revision")? != revision {
        return Err(fail(
            "Quire source premise differs from the selected candidate",
        ));
    }
    if member(premises, "schema_version")? != "tl-syntax.source-grounding-premises/v1" {
        return Err(fail("unknown source-grounding premise version"));
    }
    if member(export, "modules")? != &json!([member(premises, "module")?]) {
        return Err(fail(
            "Quire module or schema digest differs from the released premise",
        ));
    }
    if array(member(export, "obligations")?, "obligations")?.is_empty() {
        return Err(fail("Quire source export contains no obligations"));
    }
    Ok(())
}

fn validate_source_locator(
    export: &Value,
    identity: &str,
    actual_digest: &str,
) -> Result<(), String> {
    let Some(spec_path) = identity.strip_prefix("spec/") else {
        return Ok(());
    };
    let matches: Vec<_> = array(member(export, "artifacts")?, "artifacts")?
        .iter()
        .filter(|artifact| {
            artifact.pointer("/locator/path").and_then(Value::as_str) == Some(spec_path)
        })
        .collect();
    if matches.len() != 1 {
        return Err(fail(format!(
            "expected one Quire source locator for {identity}, got {}",
            matches.len()
        )));
    }
    if matches[0]
        .pointer("/locator/digest")
        .and_then(Value::as_str)
        != Some(actual_digest)
    {
        return Err(fail(format!(
            "Quire source digest differs from bytes at {identity}"
        )));
    }
    Ok(())
}

fn validate_requirement_statements(record: &Value, export: &Value) -> Result<(), String> {
    let requirements = array(
        &record["definition"]["requirements"],
        "definition.requirements",
    )?;
    let obligations = array(member(export, "obligations")?, "obligations")?;
    for requirement in requirements {
        let id = string(member(requirement, "id")?, "requirement.id")?;
        let statement = string(member(requirement, "statement")?, "requirement.statement")?;
        let found: Vec<_> = obligations
            .iter()
            .filter(|obligation| obligation["id"] == id)
            .collect();
        if found.len() != 1 {
            return Err(fail(format!(
                "expected one authoritative Quire obligation for {id}"
            )));
        }
        let source_path = format!(
            "spec/{}",
            string(member(found[0], "document")?, "obligation.document")?
        );
        let source_ids = array(member(requirement, "source_ids")?, "requirement.source_ids")?;
        if found[0]["statement"] != statement
            || !source_ids.iter().any(|source| source == &source_path)
        {
            return Err(fail(format!(
                "declaration diverges from authoritative obligation {id}"
            )));
        }
    }
    Ok(())
}

fn project(
    root: &Path,
    declaration: &Value,
    export: &Value,
    premises: &Value,
    export_bytes: &[u8],
    revision: &str,
) -> Result<Value, String> {
    if revision.len() != 40
        || !revision
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(fail("candidate revision must be a full lowercase Git SHA"));
    }
    let mut record = member(declaration, "record")?.clone();
    let repository = string(&record["subject"]["repository"], "subject.repository")?;
    check_export(export, premises, repository, revision)?;
    let sources = member(declaration, "sources")?
        .as_object()
        .ok_or_else(|| fail("sources must be an object"))?;

    let connections = record["source_connections"]
        .as_array_mut()
        .ok_or_else(|| fail("source_connections must be an array"))?;
    if connections.len() != sources.len() {
        return Err(fail(
            "source lookup and sealed connections differ in population",
        ));
    }
    let mut seen = BTreeSet::new();
    for connection in connections {
        let identity = string(member(connection, "source_id")?, "source_id")?.to_owned();
        if !seen.insert(identity.clone()) {
            return Err(fail(format!(
                "duplicate sealed source identity: {identity}"
            )));
        }
        if sources.get(&identity).and_then(Value::as_str) != Some(identity.as_str()) {
            return Err(fail(format!(
                "unsealed source lookup substitutes path {identity}"
            )));
        }
        let bytes = source_bytes(root, &identity)?;
        let actual_digest = digest(&bytes);
        validate_source_locator(export, &identity, &actual_digest)?;
        connection["revision"] = json!(revision);
        connection["digest"] = json!(actual_digest);
    }
    validate_requirement_statements(&record, export)?;
    record["subject"]["base_revision"] = json!(revision);
    record["impact_snapshot"]["revision"] = json!(revision);
    record["impact_snapshot"]["digest"] = json!(digest(export_bytes));
    // Quire grounds specification facts. It does not compare the author's
    // declared scope with the candidate's complete changed-path population.
    record["impact_snapshot"]["completeness"] = json!("incomplete");
    record["impact_snapshot"]["gaps"] = json!([
        "subject.scope is authorial; candidate change-footprint completeness is unverified"
    ]);
    let proofs = record["definition"]["proof_obligations"]
        .as_array_mut()
        .ok_or_else(|| fail("proof_obligations must be an array"))?;
    // The existing declaration describes the legacy `quire coverage` producer.
    // This path consumes a different, source-grounded export. Do not carry a
    // proof obligation whose declared command was never run for these bytes.
    proofs.retain(|proof| proof["proof_id"] != "PROOF-quire-static-export");
    for proof in proofs {
        let configuration = string(member(proof, "configuration")?, "configuration")?.to_owned();
        proof["configuration_digest"] = json!(digest(&source_bytes(root, &configuration)?));
        proof
            .as_object_mut()
            .ok_or_else(|| fail("proof must be an object"))?
            .remove("configuration");
    }
    let unknowns = record["definition"]["unknowns"]
        .as_array_mut()
        .ok_or_else(|| fail("unknowns must be an array"))?;
    unknowns.push(json!({
        "id":"UNKNOWN-source-export-not-attested",
        "statement":"The source-grounded Quire export is bound by digest as an impact snapshot but has no exact producer-execution attestation in this record; FR-015 freshness remains open.",
        "disposition":"open",
        "owner":"tl-syntax-source-readiness"
    }));
    record["impact_snapshot"]["identity"] = json!("quire-assurance-v1");
    Ok(record)
}

fn run() -> Result<(), String> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 6 {
        return Err(fail(
            "usage: source-grounding ROOT DECLARATION QUIRE_EXPORT PREMISES REVISION",
        ));
    }
    let root = PathBuf::from(&args[1]);
    let declaration = read_json(Path::new(&args[2]))?;
    let export_path = PathBuf::from(&args[3]);
    let export_bytes =
        fs::read(&export_path).map_err(|error| format!("{}: {error}", export_path.display()))?;
    let premises = read_json(Path::new(&args[4]))?;
    let export: Value = serde_json::from_slice(&export_bytes)
        .map_err(|error| format!("{}: {error}", export_path.display()))?;
    let revision = args[5]
        .to_str()
        .ok_or_else(|| fail("revision is not UTF-8"))?;
    let record = project(
        &root,
        &declaration,
        &export,
        &premises,
        &export_bytes,
        revision,
    )?;
    println!(
        "{}",
        serde_json::to_string(&record).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("source-grounding: {error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    // TL-63: the released source locator and the sealed path must agree with
    // the bytes actually opened; duplicated prose cannot replace an obligation.
    #[test]
    fn path_substitution_and_divergent_statement_refuse() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("tl-source-grounding-{}-{nonce}", process::id()));
        let spec = "spec/requirements/FR-001.md";
        fs::create_dir_all(root.join("spec/requirements")).unwrap();
        fs::write(root.join(spec), b"authoritative requirement").unwrap();
        fs::write(root.join("configuration.json"), b"{}").unwrap();
        let revision = "0123456789abcdef0123456789abcdef01234567";
        let module = json!({"name":"spec-artifacts-process","version":"0.2.0","schemas":[]});
        let premises =
            json!({"schema_version":"tl-syntax.source-grounding-premises/v1","module":module});
        let export = json!({
            "format":"quire-assurance", "format_version":1,
            "source":{"repository":"agent-ix/tl-syntax","revision":revision},
            "modules":[module],
            "artifacts":[{"locator":{"path":"requirements/FR-001.md","digest":digest(b"authoritative requirement")}}],
            "obligations":[{"id":"FR-001-AC-1","document":"requirements/FR-001.md","statement":"Exact source statement"}]
        });
        let declaration = json!({
            "sources":{spec:spec},
            "record":{
                "subject":{"repository":"agent-ix/tl-syntax"},
                "source_connections":[{"source_id":spec,"kind":"requirement"}],
                "impact_snapshot":{},
                "definition":{
                    "requirements":[{"id":"FR-001-AC-1","statement":"Exact source statement","source_ids":[spec]}],
                    "proof_obligations":[
                        {"proof_id":"PROOF-domain","configuration":"configuration.json"},
                        {"proof_id":"PROOF-quire-static-export","configuration":"configuration.json"}
                    ],
                    "unknowns":[]
                }
            }
        });
        let projected =
            project(&root, &declaration, &export, &premises, b"export", revision).unwrap();
        assert_eq!(projected["impact_snapshot"]["completeness"], "incomplete");
        assert_eq!(
            projected["definition"]["proof_obligations"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(projected["definition"]["unknowns"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "UNKNOWN-source-export-not-attested"
                && item["disposition"] == "open"));

        let mut changed = declaration.clone();
        changed["sources"][spec] = json!("configuration.json");
        assert!(
            project(&root, &changed, &export, &premises, b"export", revision)
                .unwrap_err()
                .contains("substitutes path")
        );

        changed = declaration.clone();
        changed["record"]["definition"]["requirements"][0]["statement"] = json!("Divergent prose");
        assert!(
            project(&root, &changed, &export, &premises, b"export", revision)
                .unwrap_err()
                .contains("diverges")
        );

        fs::write(root.join(spec), b"changed requirement").unwrap();
        assert!(
            project(&root, &declaration, &export, &premises, b"export", revision)
                .unwrap_err()
                .contains("digest differs")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
