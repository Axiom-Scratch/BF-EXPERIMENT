use std::env;
use std::path::PathBuf;
use std::process;

#[path = "../bfpp/mod.rs"]
mod bfpp;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), bfpp::Error> {
    let mut args = env::args().skip(1);
    let input = match args.next() {
        Some(value) => value,
        None => return Err(bfpp::Error::Usage),
    };
    if args.next().as_deref() != Some("-o") {
        return Err(bfpp::Error::Usage);
    }
    let output = match args.next() {
        Some(value) => value,
        None => return Err(bfpp::Error::Usage),
    };
    if args.next().is_some() {
        return Err(bfpp::Error::Usage);
    }

    let input_path = PathBuf::from(input);
    let output_path = PathBuf::from(output);

    let processed = bfpp::preprocess(&input_path)?;

    std::fs::write(&output_path, processed)
        .map_err(|e| bfpp::Error::WriteFailed { path: output_path, source: e })?;

    Ok(())
}
