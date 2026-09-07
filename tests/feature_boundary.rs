use std::{
    env, fs,
    io::ErrorKind,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct TargetDir(PathBuf);

impl TargetDir {
    fn create() -> Self {
        let base = env::temp_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        for attempt in 0..1000_u32 {
            let path = base.join(format!(
                "tl-syntax-feature-boundary-{}-{timestamp}-{attempt}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot create isolated target directory: {error}"),
            }
        }
        panic!("cannot allocate a unique isolated target directory")
    }
}

impl Drop for TargetDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// Trace: TC-019, TC-033, NFR-001-AC-1, NFR-001-AC-2
#[test]
fn no_std_feature_matrix_and_default_dependency_gate_execute() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target = TargetDir::create();
    let output = Command::new("make")
        .args([
            "--no-print-directory",
            "check-features",
            "check-default-dependencies",
        ])
        .env("CARGO_TARGET_DIR", &target.0)
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "feature boundary failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
