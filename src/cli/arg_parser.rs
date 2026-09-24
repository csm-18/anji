use crate::{cli::response_file_parser::expand_response_files, diagnostics::Diagnostic};

/// Parses command-line arguments.
pub fn parse(args: &mut Vec<String>) {
    // This function should return 4 things:
    //  errors
    let mut errors: Vec<Option<Diagnostic>> = vec![];
    //  filenames
    let mut filenames: Vec<String>;
    //  normal_options
    //  watch_options

    // Expand all response files and report any errors
    let response_file_errors = expand_response_files(args);
    let circular_reference_error: bool = response_file_errors
        .iter()
        .flatten()
        .any(|error| error.code == 100000);
    errors.extend(response_file_errors);
    if circular_reference_error {
        // If circular reference error exists then return
        // dbg!("return here!");
        errors
            .iter()
            .flatten()
            .find(|error| error.code == 100000)
            .unwrap()
            .print();
    } else {
        if errors.is_empty() {
            dbg!(args);
        } else {
            for err in errors {
                err.unwrap().print();
            }
        }
    }
}
