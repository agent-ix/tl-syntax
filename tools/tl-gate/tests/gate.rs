//! TL-371/TL-370: behavior of `tl-gate gate` on synthetic cargo-mutants outcomes.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The gate's exit status and standard output for one synthetic run.
struct Run {
    status: i32,
    stdout: String,
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tl-mutation-gate-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn outcome(summary: &str, name: &str) -> String {
    format!(
        r#"{{"scenario":{{"Mutant":{{"file":"src/lib.rs","name":"src/lib.rs:1:1: {name}"}}}},"summary":"{summary}"}}"#
    )
}

fn write_outcomes(dir: &Path, outcomes: &[String]) {
    let baseline = r#"{"scenario":"Baseline","summary":"Success"}"#.to_owned();
    let all = std::iter::once(baseline)
        .chain(outcomes.iter().cloned())
        .collect::<Vec<_>>()
        .join(",");
    fs::write(
        dir.join("outcomes.json"),
        format!(r#"{{"outcomes":[{all}]}}"#),
    )
    .unwrap();
}

fn thresholds(dir: &Path, floor: f64, survivors: &str) -> PathBuf {
    let path = dir.join("thresholds.toml");
    fs::write(
        &path,
        format!(
            "diff_min_kill_percent = 100.0\nrequire_survivor_reasons = true\n[[group]]\nname = \"crate\"\nglobs = [\"src/*.rs\"]\n\
             min_kill_percent = {floor}\n{survivors}"
        ),
    )
    .unwrap();
    path
}

fn gate(diff: bool, dir: &Path, thresholds: &Path) -> Run {
    let mut command = Command::new(env!("CARGO_BIN_EXE_tl-gate"));
    command.arg("gate");
    if diff {
        command.arg("--diff");
    }
    let output = command.arg(dir).arg(thresholds).output().unwrap();
    Run {
        status: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
    }
}

const LISTED_MISS: &str = "[[survivor]]\nfile = \"src/lib.rs\"\nreason = \"equivalent\"\nnames = [\"replace a with b in f\"]\n";

#[test]
fn an_unlisted_missed_mutant_fails_the_full_and_the_diff_gate() {
    let dir = scratch("unlisted");
    write_outcomes(
        &dir,
        &[
            outcome("CaughtMutant", "x"),
            outcome("MissedMutant", "replace a with b in f"),
        ],
    );
    let toml = thresholds(&dir, 0.0, "");
    for diff in [false, true] {
        let run = gate(diff, &dir, &toml);
        assert_eq!(run.status, 1, "diff={diff}");
        assert!(
            run.stdout.contains("survivor without a reason"),
            "{}",
            run.stdout
        );
    }
}

#[test]
fn a_listed_survivor_passes_and_one_with_an_empty_reason_fails() {
    let dir = scratch("listed");
    write_outcomes(
        &dir,
        &[
            outcome("CaughtMutant", "x"),
            outcome("MissedMutant", "replace a with b in f"),
        ],
    );
    let listed = thresholds(&dir, 50.0, LISTED_MISS);
    assert_eq!(gate(false, &dir, &listed).status, 0);
    let blank = thresholds(&dir, 50.0, &LISTED_MISS.replace("equivalent", " "));
    assert_eq!(gate(false, &dir, &blank).status, 1);
}

#[test]
fn a_listed_timeout_counts_against_the_kill_rate() {
    let dir = scratch("timeout");
    write_outcomes(
        &dir,
        &[
            outcome("CaughtMutant", "x"),
            outcome("Timeout", "replace a with b in f"),
        ],
    );
    let met = thresholds(&dir, 50.0, LISTED_MISS);
    assert_eq!(gate(false, &dir, &met).status, 0);
    let above = thresholds(&dir, 60.0, LISTED_MISS);
    let run = gate(false, &dir, &above);
    assert_eq!(run.status, 1);
    assert!(run.stdout.contains("timeout=1"), "{}", run.stdout);
}

#[test]
fn an_empty_full_population_fails_but_an_empty_diff_passes() {
    let dir = scratch("empty");
    write_outcomes(&dir, &[outcome("Unviable", "x")]);
    let toml = thresholds(&dir, 0.0, "");
    assert_eq!(gate(false, &dir, &toml).status, 1);
    assert_eq!(gate(true, &dir, &toml).status, 0);
}

#[test]
fn missing_outcomes_are_an_input_error_in_a_full_run_and_an_empty_diff_otherwise() {
    let dir = scratch("missing");
    let toml = thresholds(&dir, 0.0, "");
    assert_eq!(gate(false, &dir, &toml).status, 2);
    assert_eq!(gate(true, &dir, &toml).status, 0);
}

#[test]
fn survivor_listing_is_required_only_when_the_thresholds_ask_for_it() {
    let dir = scratch("optional");
    write_outcomes(
        &dir,
        &[outcome("CaughtMutant", "x"), outcome("MissedMutant", "y")],
    );
    let path = dir.join("loose.toml");
    fs::write(
        &path,
        "diff_min_kill_percent = 100.0\n[[group]]\nname = \"crate\"\nglobs = [\"src/*.rs\"]\nmin_kill_percent = 50.0\n",
    )
    .unwrap();
    assert_eq!(gate(false, &dir, &path).status, 0);
}

#[test]
fn a_failing_baseline_refuses_the_campaign_and_prints_no_kill_rate() {
    let dir = scratch("baseline");
    fs::write(
        dir.join("outcomes.json"),
        format!(
            r#"{{"outcomes":[{{"scenario":"Baseline","summary":"Failure"}},{}]}}"#,
            outcome("CaughtMutant", "x")
        ),
    )
    .unwrap();
    let toml = thresholds(&dir, 0.0, "");
    for diff in [false, true] {
        let run = gate(diff, &dir, &toml);
        assert_eq!(run.status, 1, "diff={diff}");
        assert!(run.stdout.contains("FAIL baseline"), "{}", run.stdout);
        assert!(!run.stdout.contains("kill"), "{}", run.stdout);
    }
}
