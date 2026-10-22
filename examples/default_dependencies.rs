//! Check the default normal dependency graph of the published crate.

use std::{env, fs, process::Command};

enum GateError {
    Usage,
    Failure(String),
}

fn check_tree(tree: &str) -> Result<(), String> {
    let lines: Vec<_> = tree
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    if lines.len() == 1 && lines[0].starts_with("tl-syntax v") {
        Ok(())
    } else {
        Err(format!(
            "default normal dependency graph is not empty:\n{}",
            lines.join("\n")
        ))
    }
}

fn run() -> Result<(), GateError> {
    let mut arguments = env::args().skip(1);
    let tree = match (
        arguments.next().as_deref(),
        arguments.next(),
        arguments.next(),
    ) {
        (None, None, None) => {
            let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
            let output = Command::new(cargo)
                .args([
                    "tree",
                    "--no-default-features",
                    "--edges",
                    "normal",
                    "--prefix",
                    "none",
                ])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .map_err(|error| GateError::Failure(format!("cargo tree unavailable: {error}")))?;
            if !output.status.success() {
                return Err(GateError::Failure(format!(
                    "cargo tree unavailable: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                )));
            }
            String::from_utf8(output.stdout).map_err(|error| {
                GateError::Failure(format!("cargo tree emitted invalid UTF-8: {error}"))
            })?
        }
        (Some("--tree-output"), Some(path), None) => fs::read_to_string(path)
            .map_err(|error| GateError::Failure(format!("cannot read tree output: {error}")))?,
        _ => return Err(GateError::Usage),
    };
    check_tree(&tree).map_err(GateError::Failure)
}

fn main() {
    match run() {
        Ok(()) => println!("default normal dependency graph contains no dependencies"),
        Err(GateError::Usage) => {
            eprintln!("usage: default_dependencies [--tree-output FILE]");
            std::process::exit(2);
        }
        Err(GateError::Failure(error)) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::check_tree;

    // Trace: NFR-001-AC-1, NFR-001-AC-2
    #[test]
    fn dependency_graph_rejects_an_added_normal_dependency() {
        assert!(check_tree("tl-syntax v0.3.0\n").is_ok());
        assert!(check_tree("tl-syntax v0.3.0\nserde v1.0\n").is_err());
    }
}
