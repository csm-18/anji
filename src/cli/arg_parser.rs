use crate::host;

/// Parses command-line arguments.
pub fn parse(args: &mut Vec<String>) {
    // This function should return:
    //  errors
    //  filenames
    //  normal_options
    //  watch_options

    match host::read_file("hello.txt") {
        Ok(bytes) => {
            dbg!(bytes);
        }
        Err(error) => {
            if let Some(err) = error {
                err.print();
            }
        }
    }
}
