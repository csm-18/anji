use std::env;
use std::fs;

use crate::diagnostics::create_diagnostic;
use crate::diagnostics::Diagnostic;

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

/* Environment variables */
pub fn get_env_variable(name: &str) -> bool {
    match env::var(name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/* File IO*/
pub fn read_file(path: &str) -> Result<Vec<u8>, Option<Diagnostic>> {
    let content: Vec<u8>;
    match fs::read(path) {
        Ok(bytes) => {
            content = bytes;
        }

        Err(_) => {
            // Cannot read file error
            return Err(create_diagnostic(5083, &[path]));
        }
    };

    // Check file content is valid utf-8 text
    match std::str::from_utf8(&content) {
        Ok(_) => {}
        Err(_) => {
            // Cannot read file error
            return Err(create_diagnostic(5083, &[path]));
        }
    };

    Ok(content)
}
