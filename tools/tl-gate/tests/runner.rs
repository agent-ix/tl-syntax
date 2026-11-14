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

fn set_runs(dir: &Path, runs: u32) {
    let path = dir.join("mutation-thresholds.toml");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("baseline_runs = {runs}\n{text}")).unwrap();
}

/// Extra baseline runs the fake `cargo test` counted; `None` when it never ran.
fn extra_runs(dir: &Path) -> Option<String> {
    fs::read_to_string(dir.join("fake-baseline.count")).ok()
}

#[test]
fn cargo_mutants_keeps_its_own_baseline_and_one_extra_run_is_added_by_default() {
    let dir = scratch("baseline-default");
    configure(&dir, 0, Some(CAUGHT));
    assert_eq!(
        mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap(),
        0
    );
    assert_eq!(extra_runs(&dir).as_deref(), Some("1"));
    let args = fs::read_to_string(dir.join("out/args.txt")).unwrap();
    assert!(!args.contains("--baseline"), "{args}");
}

#[test]
fn baseline_runs_is_read_from_the_thresholds_file() {
    let dir = scratch("baseline-three");
    set_runs(&dir, 3);
    configure(&dir, 0, Some(CAUGHT));
    mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap();
    assert_eq!(extra_runs(&dir).as_deref(), Some("2"));
}

#[test]
fn a_single_baseline_run_adds_no_extra_run() {
    let dir = scratch("baseline-one");
    set_runs(&dir, 1);
    configure(&dir, 0, Some(CAUGHT));
    mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap();
    assert_eq!(extra_runs(&dir), None);
}

#[test]
fn an_intermittent_extra_baseline_refuses_the_campaign_without_running_it() {
    for statuses in ["0 1", "1 0", "1 1"] {
        for base in [None, Some("HEAD")] {
            let dir = scratch("baseline-flaky");
            set_runs(&dir, 3);
            git(&dir, &["init", "-q"]);
            git(&dir, &["add", "-A"]);
            git(&dir, &["commit", "-q", "-m", "base"]);
            set_baseline(&dir, statuses);
            configure(&dir, 0, Some(CAUGHT));
            let status = mutants_in(&dir, OsStr::new(FAKE), "out", &options(base)).unwrap();
            assert_eq!(status, 1, "{statuses} base={base:?}");
            assert!(!dir.join("out/args.txt").exists(), "campaign ran");
        }
    }
}

#[test]
fn a_failing_single_extra_baseline_refuses_the_campaign() {
    let dir = scratch("baseline-fails");
    set_baseline(&dir, "1");
    configure(&dir, 0, Some(CAUGHT));
    assert_eq!(
        mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap(),
        1
    );
    assert!(!dir.join("out/args.txt").exists(), "campaign ran");
}

#[test]
fn a_diff_with_no_mutants_pays_for_no_extra_baseline_run() {
    let dir = scratch("baseline-no-mutants");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "base"]);
    fs::write(dir.join("fake-list.txt"), "").unwrap();
    configure(&dir, 0, None);
    let status = mutants_in(&dir, OsStr::new(FAKE), "out", &options(Some("HEAD"))).unwrap();
    assert_eq!(status, 0);
    assert_eq!(extra_runs(&dir), None);
}

#[test]
fn a_diff_with_mutants_pays_for_the_extra_baseline_run() {
    let dir = scratch("baseline-diff-mutants");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-q", "-m", "base"]);
    configure(&dir, 0, Some(CAUGHT));
    mutants_in(&dir, OsStr::new(FAKE), "out", &options(Some("HEAD"))).unwrap();
    assert_eq!(extra_runs(&dir).as_deref(), Some("1"));
}

#[test]
fn the_extra_run_command_is_derived_from_a_tl_mltl_shaped_config() {
    // Shaped like tl-mltl's .cargo/mutants.toml: extra cargo args, test args after `--`, and keys
    // that do not affect the test selection.
    let config = r#"
exclude_globs = ["src/ci_guard.rs"]
additional_cargo_args = ["--all-features"]
minimum_test_timeout = 300
timeout_multiplier = 3.0
exclude_re = ['^src/a\.rs:\d+:\d+: replace < with <= in f$']
additional_cargo_test_args = ["--", "--skip", "external_consumer_feature_tree_and_complete_bounded_bytes", "--skip", "mutation_gate_"]
"#;
    let dir = scratch("derived");
    fs::create_dir_all(dir.join(".cargo")).unwrap();
    fs::write(dir.join(".cargo/mutants.toml"), config).unwrap();
    configure(&dir, 0, Some(CAUGHT));
    mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap();
    let ran = fs::read_to_string(dir.join("fake-baseline.args")).unwrap();
    assert_eq!(
        ran,
        "test --all-features -- --skip external_consumer_feature_tree_and_complete_bounded_bytes --skip mutation_gate_"
    );
}

#[test]
fn a_config_the_extra_runs_cannot_reproduce_is_refused_before_any_run() {
    for config in ["test_tool = \"nextest\"\n", "test_package = [\"a\"]\n"] {
        let dir = scratch("irreproducible");
        fs::create_dir_all(dir.join(".cargo")).unwrap();
        fs::write(dir.join(".cargo/mutants.toml"), config).unwrap();
        configure(&dir, 0, Some(CAUGHT));
        let error = mutants_in(&dir, OsStr::new(FAKE), "out", &options(None)).unwrap_err();
        assert!(error.contains("not supported"), "{error}");
        assert!(!dir.join("out/args.txt").exists(), "campaign ran");
    }
}
