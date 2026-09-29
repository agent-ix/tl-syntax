//! FR-006 domain-only transcription of existing corpus results to Quoin entries.
//! No producer, subprocess, retention or release decision is owned here.

#![forbid(unsafe_code)]

use serde_json::{json, Value};
use std::{
    env, fs,
    io::{self, Write},
    process,
};

const PROTOCOL: &str = "tl-syntax.corpus-conformance/v1";

#[derive(Debug)]
struct AdapterError {
    code: &'static str,
    detail: String,
}

impl AdapterError {
    fn new(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

fn adapt(raw: &str) -> Result<Value, AdapterError> {
    let mut entries = Vec::new();
    for (index, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row: Value = serde_json::from_str(line).map_err(|error| {
            AdapterError::new("malformed_row", format!("line {}: {error}", index + 1))
        })?;
        if row["protocol"] != PROTOCOL {
            return Err(AdapterError::new(
                "foreign_protocol",
                format!("line {} does not declare {PROTOCOL}", index + 1),
            ));
        }
        let outcome = match row["outcome"].as_str() {
            Some("pass") => "pass",
            Some("fail" | "malformed") => "fail",
            Some("unavailable" | "not-computed" | "vacuous") => "skip",
            _ => {
                return Err(AdapterError::new(
                    "unnamed_outcome",
                    format!("line {} declares an unknown or absent outcome", index + 1),
                ))
            }
        };
        let symbol = row["symbol"]
            .as_str()
            .filter(|symbol| !symbol.is_empty())
            .ok_or_else(|| {
                AdapterError::new(
                    "invalid_symbol",
                    format!("line {} requires a nonempty symbol", index + 1),
                )
            })?;
        let trace_ids = match row.get("traceIds") {
            None => Vec::new(),
            Some(Value::Array(ids)) if ids.iter().all(Value::is_string) => ids.clone(),
            _ => {
                return Err(AdapterError::new(
                    "invalid_traces",
                    format!("line {} traceIds must be an array of strings", index + 1),
                ))
            }
        };
        entries.push(json!({"symbol":symbol,"outcome":outcome,"traceIds":trace_ids}));
    }
    if entries.is_empty() {
        return Err(AdapterError::new(
            "empty_stream",
            "no conformance rows were supplied",
        ));
    }
    Ok(json!({"entries":entries}))
}

fn run() -> Result<(), AdapterError> {
    let mut args = env::args_os().skip(1);
    let input = args
        .next()
        .ok_or_else(|| AdapterError::new("usage", "usage: conformance-adapter INPUT.jsonl"))?;
    if args.next().is_some() {
        return Err(AdapterError::new(
            "usage",
            "usage: conformance-adapter INPUT.jsonl",
        ));
    }
    let raw = fs::read_to_string(&input)
        .map_err(|error| AdapterError::new("input_unavailable", error.to_string()))?;
    // Validate the whole stream before exposing any normalized entries.
    let entries = adapt(&raw)?;
    let bytes = serde_json::to_vec(&entries)
        .map_err(|error| AdapterError::new("serialization", error.to_string()))?;
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&bytes)
        .and_then(|()| stdout.write_all(b"\n"))
        .map_err(|error| AdapterError::new("output_unavailable", error.to_string()))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("conformance-adapter: {}: {}", error.code, error.detail);
        process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_outcomes_keep_the_legacy_quoin_mapping() {
        for (source, expected) in [
            ("pass", "pass"),
            ("fail", "fail"),
            ("malformed", "fail"),
            ("unavailable", "skip"),
            ("not-computed", "skip"),
            ("vacuous", "skip"),
        ] {
            let row = json!({"protocol":PROTOCOL,"symbol":"corpus::case::check","outcome":source,"traceIds":["TC-022","FR-006-AC-2"],"detail":{"kept_by_producer":true}});
            let result = adapt(&row.to_string()).unwrap();
            assert_eq!(result["entries"][0]["outcome"], expected);
            assert_eq!(result["entries"][0]["traceIds"], row["traceIds"]);
        }
    }

    #[test]
    fn malformed_later_rows_never_expose_a_successful_prefix() {
        let healthy = json!({"protocol":PROTOCOL,"symbol":"control","outcome":"pass"});
        assert!(adapt(&healthy.to_string()).is_ok());
        for (row, code) in [
            ("", "empty_stream"),
            ("{", "malformed_row"),
            (
                r#"{"protocol":"foreign","symbol":"x","outcome":"pass"}"#,
                "foreign_protocol",
            ),
            (
                r#"{"protocol":"tl-syntax.corpus-conformance/v1","symbol":"x","outcome":"unknown"}"#,
                "unnamed_outcome",
            ),
        ] {
            assert_eq!(adapt(row).unwrap_err().code, code);
            if !row.is_empty() {
                assert_eq!(adapt(&format!("{healthy}\n{row}")).unwrap_err().code, code);
            }
        }
    }
}
