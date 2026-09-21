/// Returns command-line arguments.
pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}
