use std::env;
use std::process;

use bf::run_file;

const DEFAULT_TAPE_SIZE: usize = 30_000;
const TAPE_FLAG: &str = "--tape";
const TAPE_FLAG_EQ: &str = "--tape=";
const MAX_STEPS_FLAG: &str = "--max-steps";
const MAX_STEPS_FLAG_EQ: &str = "--max-steps=";
const DUMP_IR_FLAG: &str = "--dump-ir";
const TRACE_FLAG: &str = "--trace";
const NO_OPT_FLAG: &str = "--no-opt";
const EXIT_USAGE: i32 = 2;
const EXIT_RUNTIME: i32 = 1;
const MAX_STEPS_ERROR: &str = "max steps";

fn usage() -> &'static str {
    "Usage: bf <file> [--tape N] [--max-steps N] [--dump-ir] [--trace] [--no-opt]"
}

fn parse_tape_size(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|e| format!("invalid tape size '{}': {}", value, e))?;
    if parsed == 0 {
        Err("tape size must be greater than 0".to_string())
    } else {
        Ok(parsed)
    }
}

fn parse_max_steps(value: &str) -> Result<u64, String> {
    let parsed = value
        .parse::<u64>()
        .map_err(|e| format!("invalid max steps '{}': {}", value, e))?;
    if parsed == 0 {
        Err("max steps must be greater than 0".to_string())
    } else {
        Ok(parsed)
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let mut tape_size = DEFAULT_TAPE_SIZE;
    let mut tape_specified = false;
    let mut max_steps = None;
    let mut max_steps_specified = false;
    let mut dump_ir = false;
    let mut trace = false;
    let mut no_opt = false;
    let mut path = None;

    while let Some(arg) = args.next() {
        if arg == TAPE_FLAG {
            let value = match args.next() {
                Some(value) => value,
                None => {
                    eprintln!("{}: missing value for {}", usage(), TAPE_FLAG);
                    process::exit(EXIT_USAGE);
                }
            };
            if tape_specified {
                eprintln!("{}: tape size already set", usage());
                process::exit(EXIT_USAGE);
            }
            tape_size = match parse_tape_size(&value) {
                Ok(size) => size,
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(EXIT_USAGE);
                }
            };
            tape_specified = true;
            continue;
        }

        if let Some(rest) = arg.strip_prefix(TAPE_FLAG_EQ) {
            if tape_specified {
                eprintln!("{}: tape size already set", usage());
                process::exit(EXIT_USAGE);
            }
            tape_size = match parse_tape_size(rest) {
                Ok(size) => size,
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(EXIT_USAGE);
                }
            };
            tape_specified = true;
            continue;
        }

        if arg == MAX_STEPS_FLAG {
            let value = match args.next() {
                Some(value) => value,
                None => {
                    eprintln!("{}: missing value for {}", usage(), MAX_STEPS_FLAG);
                    process::exit(EXIT_USAGE);
                }
            };
            if max_steps_specified {
                eprintln!("{}: max steps already set", usage());
                process::exit(EXIT_USAGE);
            }
            max_steps = match parse_max_steps(&value) {
                Ok(limit) => Some(limit),
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(EXIT_USAGE);
                }
            };
            max_steps_specified = true;
            continue;
        }

        if let Some(rest) = arg.strip_prefix(MAX_STEPS_FLAG_EQ) {
            if max_steps_specified {
                eprintln!("{}: max steps already set", usage());
                process::exit(EXIT_USAGE);
            }
            max_steps = match parse_max_steps(rest) {
                Ok(limit) => Some(limit),
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(EXIT_USAGE);
                }
            };
            max_steps_specified = true;
            continue;
        }

        if arg == DUMP_IR_FLAG {
            dump_ir = true;
            continue;
        }

        if arg == TRACE_FLAG {
            trace = true;
            continue;
        }

        if arg == NO_OPT_FLAG {
            no_opt = true;
            continue;
        }

        if arg.starts_with('-') {
            eprintln!("{}", usage());
            process::exit(EXIT_USAGE);
        }

        if path.is_some() {
            eprintln!("{}", usage());
            process::exit(EXIT_USAGE);
        }
        path = Some(arg);
    }

    let path = match path {
        Some(path) => path,
        None => {
            eprintln!("{}", usage());
            process::exit(EXIT_USAGE);
        }
    };

    if let Err(err) = run_file(&path, tape_size, max_steps, dump_ir, trace, no_opt) {
        eprintln!("{}", err);
        if err.contains(MAX_STEPS_ERROR) {
            process::exit(1);
        }
        process::exit(EXIT_RUNTIME);
    }
}
