//! `tl-gate mutants`: run cargo-mutants, then the gate on its output.

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::gate;

const USAGE: &str = "usage: tl-gate mutants [--diff-base REF] [--jobs N] [--source-env] OUT_DIR";

fn capture(program: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    if !out.status.success() {
        return Err(format!("{program} {args:?} failed with {}", out.status));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// Without `--diff-base` this is the full run. With it only mutants in
/// `REF...HEAD` are generated, so only committed changes are mutated.
/// cargo-mutants exits 2 when mutants survive and 3 when some time out; the
/// gate judges both, so 0, 2 and 3 are accepted and any other status is passed
/// on. `--source-env` exports `TL_MLTL_SOURCE_REVISION`/`TL_MLTL_SOURCE_STATE`
/// for crates whose build.rs reads git, because cargo-mutants builds in a copy
/// of the tree without `.git`.
pub(crate) fn command(args: Vec<String>) -> u8 {
    let mut diff_base = None;
    let mut jobs = "4".to_owned();
    let mut source_env = false;
    let mut rest = args.into_iter();
    let mut positional = Vec::new();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--diff-base" => diff_base = rest.next(),
            "--jobs" => match rest.next() {
                Some(j) => jobs = j,
                None => {
                    eprintln!("{USAGE}");
                    return 2;
                }
            },
            "--source-env" => source_env = true,
            flag if flag.starts_with("--") => {
                eprintln!("{USAGE}");
                return 2;
            }
            _ => positional.push(arg),
        }
    }
    if positional.len() != 1 {
        eprintln!("{USAGE}");
        return 2;
    }
    match run(&positional[0], diff_base.as_deref(), &jobs, source_env) {
        Ok(status) => status,
        Err(e) => {
            eprintln!("run_mutants: {e}");
            2
        }
    }
}

fn run(out_dir: &str, diff_base: Option<&str>, jobs: &str, source_env: bool) -> Result<u8, String> {
    fs::create_dir_all(out_dir).map_err(|e| format!("{out_dir}: {e}"))?;
    let mut command = Command::new("cargo");
    command.args(["mutants", "-j", jobs, "--no-shuffle", "--output", out_dir]);
    if source_env {
        let dirty = capture("git", &["status", "--porcelain", "--untracked-files=no"])?;
        command
            .env(
                "TL_MLTL_SOURCE_REVISION",
                capture("git", &["rev-parse", "HEAD"])?,
            )
            .env(
                "TL_MLTL_SOURCE_STATE",
                if dirty.is_empty() {
                    "clean"
                } else {
                    "modified"
                },
            );
    }
    if let Some(base) = diff_base {
        let diff = capture("git", &["diff", &format!("{base}...HEAD")])?;
        let path = Path::new(out_dir).join("changes.diff");
        // `git diff` output ends in a newline that `capture` trimmed.
        fs::write(&path, format!("{diff}\n")).map_err(|e| format!("{}: {e}", path.display()))?;
        command.arg("--in-diff").arg(&path);
    }
    let status = command
        .status()
        .map_err(|e| format!("cargo mutants: {e}"))?;
    match status.code() {
        Some(0 | 2 | 3) => {}
        other => {
            eprintln!("run_mutants: cargo mutants failed with status {other:?}");
            return Ok(other.map_or(1, |c| u8::try_from(c).unwrap_or(1)));
        }
    }
    let mut gate_args = Vec::new();
    if diff_base.is_some() {
        gate_args.push("--diff".to_owned());
    }
    gate_args.push(format!("{out_dir}/mutants.out"));
    Ok(gate::command(gate_args))
}
