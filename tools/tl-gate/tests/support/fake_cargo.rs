//! Stand-in for `cargo mutants` used by the runner tests. It reads
//! `fake-cargo.json` from the `--output` directory: `{"exit": N, "outcomes":
//! "<json or null>"}`, writes `mutants.out/outcomes.json` when given, and
//! records its arguments in `args.txt`. Any other command is a baseline test run: it exits with the
//! next status listed in `fake-baseline.txt` in the working directory (0 when there is no file),
//! counting runs in `fake-baseline.count`.

use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list") {
        // `cargo mutants --list`: print `fake-list.txt` (one mutant per line) or a single mutant.
        print!(
            "{}",
            fs::read_to_string("fake-list.txt")
                .unwrap_or_else(|_| "src/lib.rs:1:1: x\n".to_owned())
        );
        exit(0);
    }
    if args.first().map(String::as_str) != Some("mutants") {
        baseline_run();
    }
    let out = args
        .iter()
        .position(|a| a == "--output")
        .and_then(|i| args.get(i + 1))
        .expect("--output DIR");
    fs::write(format!("{out}/args.txt"), args.join("\n")).unwrap();
    let config = fs::read_to_string(format!("{out}/fake-cargo.json")).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config).unwrap();
    if let Some(outcomes) = config["outcomes"].as_str() {
        fs::create_dir_all(format!("{out}/mutants.out")).unwrap();
        fs::write(format!("{out}/mutants.out/outcomes.json"), outcomes).unwrap();
    }
    exit(i32::try_from(config["exit"].as_i64().unwrap()).unwrap());
}

fn baseline_run() -> ! {
    let statuses = fs::read_to_string("fake-baseline.txt").unwrap_or_default();
    let statuses: Vec<i32> = statuses
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let done: usize = fs::read_to_string("fake-baseline.count")
        .map(|s| s.trim().parse().unwrap())
        .unwrap_or(0);
    fs::write("fake-baseline.count", (done + 1).to_string()).unwrap();
    let args: Vec<String> = env::args().skip(1).collect();
    fs::write("fake-baseline.args", args.join(" ")).unwrap();
    exit(statuses.get(done).copied().unwrap_or(0));
}
