use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn usage() -> &'static str {
    "Usage: bfpp <input.bfpp> -o <output.bf>"
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .ok_or_else(|| usage().to_string())?;
    if args.next().as_deref() != Some("-o") {
        return Err(usage().to_string());
    }
    let output = args.next().ok_or_else(|| usage().to_string())?;

    let mut source = Vec::new();
    File::open(&input)
        .map_err(|e| format!("failed to open '{}': {}", input, e))?
        .read_to_end(&mut source)
        .map_err(|e| format!("failed to read '{}': {}", input, e))?;

    let processed = preprocess(&source);

    let mut dest = File::create(&output)
        .map_err(|e| format!("failed to create '{}': {}", output, e))?;
    dest.write_all(&processed)
        .map_err(|e| format!("failed to write '{}': {}", output, e))?;

    Ok(())
}

fn preprocess(source: &[u8]) -> Vec<u8> {
    source.to_vec()
}
