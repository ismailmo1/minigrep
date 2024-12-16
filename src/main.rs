use ::std::env;
use minigrep::Config;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error {e}");
        process::exit(1);
    };
}
