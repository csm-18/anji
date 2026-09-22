use crate::host;
mod arg_parser;
use arg_parser::parse;

/// Parses and executes command-line arguments.
pub fn run(args: &mut Vec<String>) {
    if !args.is_empty() && is_build_command(&args[0]) {
        // Build mode
        host::writeln!("Build command is not implemented yet!");
        host::exit(1);
    } else {
        // Normal mode
        parse(args);
    }
}

fn is_build_command(name: &str) -> bool {
    let name = name.to_lowercase();
    if name == "-build" || name == "--build" || name == "-b" || name == "--b" {
        return true;
    }
    false
}
