use crate::host;

/// Parses and executes command-line arguments.
pub fn run(args: Vec<String>) {
    if !args.is_empty() && is_build_command(&args[0]) {
        // Build mode
        host::writeln!("Build command is not implemented yet!");
        host::exit(1);
    } else {
        // Normal mode
    }
}

fn is_build_command(name: &str) -> bool {
    let name = name.to_lowercase();
    if name == "-build" || name == "--build" || name == "-b" || name == "--b" {
        return true;
    }
    false
}
