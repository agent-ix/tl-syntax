//! `tl-gate mutants`: run cargo-mutants, then the gate on its output.

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::gate;

const USAGE: &str = "usage: tl-gate mutants [--diff-base REF] [--jobs N] [--source-env] OUT_DIR";

/// Options of one `mutants` run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
    /// Mutate only the committed diff `REF...HEAD`; every mutant in it must be killed.
    pub diff_base: Option<String>,
    /// Parallel cargo-mutants jobs.
    pub jobs: String,
    /// Export `TL_MLTL_SOURCE_REVISION`/`TL_MLTL_SOURCE_STATE` for build scripts that read git.
    pub source_env: bool,
}

fn capture(dir: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(program)
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    if !out.status.success() {
        return Err(format!("{program} {args:?} failed with {}", out.status));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

pub(crate) fn command(args: Vec<String>) -> u8 {
    let mut options = Options {
        diff_base: None,
        jobs: "4".to_owned(),
        source_env: false,
    };
    let mut rest = args.into_iter();
    let mut positional = Vec::new();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--diff-base" => options.diff_base = rest.next(),
            "--jobs" => match rest.next() {
                Some(j) => options.jobs = j,
                None => {
                    eprintln!("{USAGE}");
                    return 2;
                }
            },
            "--source-env" => options.source_env = true,
            flag if flag.starts_with("--") => {
                eprintln!("{USAGE}");
                return 2;
            }
            _ => positional.push(arg),
        }
    }
    if positional.len() != 1 || options.diff_base.as_deref() == Some("") {
        eprintln!("{USAGE}");
        return 2;
    }
    match mutants_in(
        Path::new("."),
        OsStr::new("cargo"),
        &positional[0],
        &options,
    ) {
        Ok(status) => status,
        Err(e) => {
            eprintln!("run_mutants: {e}");
            2
        }
    }
}

/// Run `cargo mutants` (the `cargo` program) in `dir`, writing under `out_dir`
/// (relative to `dir`), then judge `out_dir/mutants.out` against
/// `dir/mutation-thresholds.toml`.
///
/// cargo-mutants exits 2 when mutants survive and 3 when some time out; the
/// gate judges both, so 0, 2 and 3 are accepted and any other status is passed
/// on. A previous run's `outcomes.json` is removed first: cargo-mutants writes
/// none when the diff holds no mutable code, and a stale file would be judged
/// as this run's. With a diff base only committed changes are mutated.
pub fn mutants_in(
    dir: &Path,
    cargo: &OsStr,
    out_dir: &str,
    options: &Options,
) -> Result<u8, String> {
    let out_path = dir.join(out_dir);
    fs::create_dir_all(&out_path).map_err(|e| format!("{}: {e}", out_path.display()))?;
    let stale = out_path.join("mutants.out/outcomes.json");
    match fs::remove_file(&stale) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("{}: {e}", stale.display())),
    }
    let mut command = Command::new(cargo);
    command.current_dir(dir).args([
        "mutants",
        "-j",
        &options.jobs,
        "--no-shuffle",
        "--output",
        out_dir,
    ]);
    if options.source_env {
        let dirty = capture(
            dir,
            "git",
            &["status", "--porcelain", "--untracked-files=no"],
        )?;
        command
            .env(
                "TL_MLTL_SOURCE_REVISION",
                capture(dir, "git", &["rev-parse", "HEAD"])?,
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
    if let Some(base) = &options.diff_base {
        let diff = Command::new("git")
            .current_dir(dir)
            .args(["diff", &format!("{base}...HEAD")])
            .output()
            .map_err(|e| format!("git: {e}"))?;
        if !diff.status.success() {
            return Err(format!(
                "git diff {base}...HEAD failed with {}",
                diff.status
            ));
        }
        let path = out_path.join("changes.diff");
        // Written byte for byte; an empty diff is an empty file.
        fs::write(&path, &diff.stdout).map_err(|e| format!("{}: {e}", path.display()))?;
        command.arg("--in-diff").arg(&path);
    }
    let status = command
        .status()
        .map_err(|e| format!("{}: {e}", cargo.to_string_lossy()))?;
    match status.code() {
        Some(0 | 2 | 3) => {}
        other => {
            eprintln!("run_mutants: cargo mutants failed with status {other:?}");
            return Ok(other.map_or(1, |c| u8::try_from(c).unwrap_or(1)));
        }
    }
    let mut gate_args = Vec::new();
    if options.diff_base.is_some() {
        gate_args.push("--diff".to_owned());
    }
    gate_args.push(out_path.join("mutants.out").to_string_lossy().into_owned());
    gate_args.push(
        dir.join("mutation-thresholds.toml")
            .to_string_lossy()
            .into_owned(),
    );
    Ok(gate::command(gate_args))
}
