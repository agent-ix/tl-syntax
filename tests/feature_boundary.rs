use std::{env, fs, path::PathBuf, process::Command};

// Trace: TC-019, NFR-001-AC-1, NFR-001-AC-2
#[test]
fn no_std_feature_matrix_and_default_dependency_gate_execute() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target = env::temp_dir().join(format!("tl-syntax-feature-boundary-{}", std::process::id()));
    if target.exists() {
        fs::remove_dir_all(&target).unwrap();
    }
    let output = Command::new("make")
        .args([
            "--no-print-directory",
            "check-features",
            "check-default-dependencies",
        ])
        .env("CARGO_TARGET_DIR", &target)
        .current_dir(&root)
        .output()
        .unwrap();
    let cleanup = fs::remove_dir_all(&target);
    assert!(
        output.status.success(),
        "feature boundary failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    cleanup.unwrap();
}
