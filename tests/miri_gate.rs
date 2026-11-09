//! NFR-006-AC-2: the Miri undefined-behavior check is a declared, pinned `ci`
//! gate. These tests read the Makefile only; they never run Miri.

const MAKEFILE: &str = include_str!("../Makefile");

fn recipe(target: &str) -> String {
    let header = format!("{target}:");
    MAKEFILE
        .lines()
        .skip_while(|line| !line.starts_with(&header))
        .enumerate()
        .take_while(|(index, line)| *index == 0 || line.starts_with('\t'))
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join(" ")
}

// Trace: TC-164, NFR-006-AC-2
#[test]
fn miri_is_a_pinned_nightly_gate_in_the_ci_prerequisites() {
    let ci = recipe("ci");
    assert!(
        ci.split_whitespace().any(|gate| gate == "miri"),
        "ci prerequisites must include miri: {ci}"
    );

    let miri = recipe("miri");
    assert!(
        miri.starts_with("miri:"),
        "Makefile must declare a miri target"
    );
    assert!(MAKEFILE.contains("MIRI_TOOLCHAIN ?= nightly-"));
    assert!(
        miri.contains("+$(MIRI_TOOLCHAIN) miri test"),
        "miri must run on the pinned toolchain: {miri}"
    );
    assert!(
        miri.contains("MIRIFLAGS="),
        "miri must set MIRIFLAGS: {miri}"
    );
}
