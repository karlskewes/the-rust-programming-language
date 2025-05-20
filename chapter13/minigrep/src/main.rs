use std::env; // importing parent module env better than args as keeps std:: namespace prefix.
use std::process;

use minigrep::Config; // perhaps unnecessary? instead let cfg = minigrep::Config ...

fn main() {
    // args() panics if invalid unicode, use std::env::args_os instead if can't panic, but
    // returns OsString which is more complicated.
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        // print errors to stderr with eprintln! macro instead of println!
        eprintln!("Parsing arguments failed: {err}");
        process::exit(1);
    });

    // run returns Result<(), Error> so there's no useful value requiring unwrap when successful.
    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Running failed: {e}");
        process::exit(1);
    };
}
