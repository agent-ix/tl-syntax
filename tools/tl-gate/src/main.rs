//! `tl-gate gate|mutants ...`; see [`tl_gate`].

fn main() -> std::process::ExitCode {
    tl_gate::run(std::env::args().skip(1))
}
