//! Mutation gate and cargo-mutants runner for the tl-* crates (TL-370).
//!
//! One implementation serves tl-syntax, tl-parse, tl-rewrite and tl-mltl. Each
//! consumer reaches it through a small `tools/gate` shim package that depends
//! on this crate; the revision lives only in that shim's `Cargo.toml`.
//!
//! Subcommands:
//!
//! * `gate [--diff] DIR[,DIR...] [THRESHOLDS]` judges cargo-mutants
//!   `outcomes.json` files against a thresholds file.
//! * `mutants [--diff-base REF] [--jobs N] [--source-env] OUT` runs
//!   `cargo mutants` and then the gate on `OUT/mutants.out`.
//!
//! Kill rate is caught / (caught + missed + timeout); unviable mutants count on
//! neither side. A timeout counts as not killed.

mod gate;
mod runner;

pub use gate::{judge, Judgement, Mode};
pub use runner::{mutants_in, Options};
use std::process::ExitCode;

const USAGE: &str = "usage:
  tl-gate gate [--diff] DIR[,DIR...] [THRESHOLDS]
  tl-gate mutants [--diff-base REF] [--jobs N] [--source-env] OUT_DIR";

/// Run the tool on `args` (without the program name) and return its exit status.
/// Statuses: 0 pass, 1 gate failure, 2 usage or input error.
pub fn run<I: IntoIterator<Item = String>>(args: I) -> ExitCode {
    let mut args = args.into_iter();
    let code = match args.next().as_deref() {
        Some("gate") => gate::command(args.collect()),
        Some("mutants") => runner::command(args.collect()),
        _ => {
            eprintln!("{USAGE}");
            2
        }
    };
    ExitCode::from(code)
}
