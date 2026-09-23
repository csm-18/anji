use crate::diagnostics::create_diagnostic;

/// Parses command-line arguments.
pub fn parse(args: &mut Vec<String>) {
    // This function should return:
    //  errors
    //  filenames
    //  normal_options
    //  watch_options
    if let Some(error) = create_diagnostic(5083, &["hello.txt"]) {
        error.print();
    }
}
