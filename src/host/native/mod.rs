/// Returns command-line arguments.
pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

#[macro_export]
macro_rules! writeln {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}
pub use crate::writeln; // Re-export the crate-root macro from native

pub fn exit(code: i32) {
    std::process::exit(code);
}
