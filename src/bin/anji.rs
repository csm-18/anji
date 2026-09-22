use anji::cli;
use anji::host;

fn main() {
    let mut args = host::args();
    cli::run(&mut args);
}
