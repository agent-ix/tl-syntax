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

const MISSED: &str = r#"{"outcomes":[{"scenario":"Baseline","summary":"Success"},{"scenario":{"Mutant":{"file":"src/lib.rs","name":"src/lib.rs:1:1: x"}},"summary":"MissedMutant"}]}"#;

// cargo-mutants writes no outcomes when the diff holds no mutable code; the previous run's file
// must not then be judged as this run's result (SR-231 FND-2314).
#[test]
fn a_previous_runs_outcomes_are_not_judged_when_this_run_writes_none() {
    let dir = scratch("stale");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "base"]);
    fs::create_dir_all(dir.join("out/mutants.out")).unwrap();
    fs::write(dir.join("out/mutants.out/outcomes.json"), MISSED).unwrap();
    configure(&dir, 0, None);
    let status = mutants_in(&dir, OsStr::new(FAKE), "out", &options(Some("HEAD"))).unwrap();
    assert_eq!(status, 0);

    // Control: the same file is a failing verdict when this run produces it.
    let control = scratch("stale-control");
    git(&control, &["init", "-q"]);
    git(&control, &["add", "-A"]);
    git(&control, &["commit", "-q", "-m", "base"]);
    configure(&control, 2, Some(MISSED));
    let status = mutants_in(&control, OsStr::new(FAKE), "out", &options(Some("HEAD"))).unwrap();
    assert_ne!(status, 0);
}

fn set_baseline(dir: &Path, statuses: &str) {
    fs::write(dir.join("fake-baseline.txt"), statuses).unwrap();
}

fn baseline_runs_done(dir: &Path) -> String {
    fs::read_to_string(dir.join("fake-baseline.count")).unwrap()
}

#[test]
fn the_baseline_runs_twice_by_default_before_the_campaign() {
    let dir = scratch("baseline-default");
    configure(&dir, 0, Some(CAUGHT));
    assert_eq!(
        mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap(),
        0
    );
    assert_eq!(baseline_runs_done(&dir), "2");
    let args = fs::read_to_string(dir.join("out/args.txt")).unwrap();
    assert!(args.contains("--baseline\nskip"), "{args}");
}

#[test]
fn baseline_runs_is_read_from_the_thresholds_file() {
    let dir = scratch("baseline-three");
    let path = dir.join("mutation-thresholds.toml");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("baseline_runs = 3\n{text}")).unwrap();
    configure(&dir, 0, Some(CAUGHT));
    mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap();
    assert_eq!(baseline_runs_done(&dir), "3");
}

#[test]
fn an_intermittent_baseline_refuses_the_campaign_without_running_it() {
    for statuses in ["0 1", "1 0", "1 1"] {
        let dir = scratch("baseline-flaky");
        set_baseline(&dir, statuses);
        configure(&dir, 0, Some(CAUGHT));
        for base in [None, Some("HEAD")] {
            if base.is_some() {
                git(&dir, &["init", "-q"]);
                git(&dir, &["add", "-A"]);
                git(&dir, &["commit", "-q", "-m", "base"]);
                fs::remove_file(dir.join("fake-baseline.count")).unwrap();
            }
            let status = mutants_in(&dir, OsStr::new(FAKE), "out", &options(base)).unwrap();
            assert_eq!(status, 1, "{statuses} base={base:?}");
            assert!(!dir.join("out/args.txt").exists(), "campaign ran");
        }
    }
}
