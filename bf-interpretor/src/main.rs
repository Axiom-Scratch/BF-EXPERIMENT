use std::env;
use std::process;

use bf::run_file;

fn usage() -> &'static str {
    "Usage: bf <path>"
}

fn main() {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("{}", usage());
            process::exit(1);
        }
    };

    if args.next().is_some() {
        eprintln!("{}", usage());
        process::exit(1);
    }

    if let Err(err) = run_file(&path) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
