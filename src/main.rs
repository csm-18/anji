mod cli;
mod host;

fn main() {
    cli::run(host::args());
}
