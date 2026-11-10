//! Stand-in for `cargo mutants` used by the runner tests. It reads
//! `fake-cargo.json` from the `--output` directory: `{"exit": N, "outcomes":
//! "<json or null>"}`, writes `mutants.out/outcomes.json` when given, and
//! records its arguments in `args.txt`.

use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
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
