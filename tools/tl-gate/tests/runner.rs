//! `tl-gate mutants` against a stand-in `cargo mutants` (the `fake-cargo` bin).

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tl_gate::{mutants_in, Options};

const FAKE: &str = env!("CARGO_BIN_EXE_fake-cargo");

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tl-runner-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("mutation-thresholds.toml"),
        "diff_min_kill_percent = 100.0\n[[group]]\nname = \"crate\"\nglobs = [\"src/*.rs\"]\nmin_kill_percent = 50.0\n",
    )
    .unwrap();
    dir
}

fn options(diff_base: Option<&str>) -> Options {
    Options {
        diff_base: diff_base.map(str::to_owned),
        jobs: "2".to_owned(),
        source_env: false,
    }
}

fn configure(dir: &Path, exit: i32, outcomes: Option<&str>) {
    fs::create_dir_all(dir.join("out")).unwrap();
    let config = serde_json::json!({ "exit": exit, "outcomes": outcomes });
    fs::write(dir.join("out/fake-cargo.json"), config.to_string()).unwrap();
}

const CAUGHT: &str = r#"{"outcomes":[{"scenario":"Baseline","summary":"Success"},{"scenario":{"Mutant":{"file":"src/lib.rs","name":"src/lib.rs:1:1: x"}},"summary":"CaughtMutant"}]}"#;

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?}");
}

#[test]
fn survivor_status_two_is_accepted_and_the_gate_judges_the_outcomes() {
    let dir = scratch("status2");
    configure(&dir, 2, Some(CAUGHT));
    let status = mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap();
    assert_eq!(status, 0);
    let args = fs::read_to_string(dir.join("out/args.txt")).unwrap();
    assert!(
        args.contains("--no-shuffle") && !args.contains("--in-diff"),
        "{args}"
    );
}

#[test]
fn any_other_cargo_mutants_status_is_passed_on_without_judging() {
    let dir = scratch("status1");
    configure(&dir, 1, Some(CAUGHT));
    assert_eq!(
        mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap(),
        1
    );
}

#[test]
fn an_empty_diff_writes_an_empty_file_and_passes_with_no_outcomes() {
    let dir = scratch("emptydiff");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "base"]);
    configure(&dir, 0, None);
    let status = mutants_in(&dir, OsStr::new(FAKE), "out", &options(Some("HEAD"))).unwrap();
    assert_eq!(status, 0);
    assert_eq!(fs::read(dir.join("out/changes.diff")).unwrap(), b"");
    let args = fs::read_to_string(dir.join("out/args.txt")).unwrap();
    assert!(args.contains("--in-diff"), "{args}");
}

#[test]
fn the_diff_is_written_byte_for_byte() {
    let dir = scratch("bytes");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "base"]);
    // Invalid UTF-8 and trailing blank lines must survive unchanged.
    fs::write(dir.join("data.bin"), b"a\xff\xfeb\n\n\n").unwrap();
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "change"]);
    configure(&dir, 0, None);
    mutants_in(&dir, OsStr::new(FAKE), "out", &options(Some("HEAD~1"))).unwrap();
    let written = fs::read(dir.join("out/changes.diff")).unwrap();
    let expected = Command::new("git")
        .current_dir(&dir)
        .args(["diff", "HEAD~1...HEAD"])
        .output()
        .unwrap()
        .stdout;
    assert_eq!(written, expected);
    assert!(written.windows(2).any(|w| w == b"\xff\xfe"));
}
