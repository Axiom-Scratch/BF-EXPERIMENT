use std::path::Path;

use super::error::Error;
use super::include;
use super::repeat;

pub fn preprocess(input_path: &Path) -> Result<String, Error> {
    let resolved = include::resolve_includes(input_path)?;
    let stripped = strip_comments(&resolved);
    let expanded = repeat::expand_repeats(&stripped)?;
    Ok(expanded)
}

fn strip_comments(text: &str) -> String {
    let mut out = String::new();
    for part in text.split_inclusive('\n') {
        let has_newline = part.ends_with('\n');
        let mut line = &part[..part.len() - if has_newline { 1 } else { 0 }];
        if line.ends_with('\r') {
            line = &line[..line.len() - 1];
        }
        let trimmed = line.trim_start();
        let is_include = trimmed.starts_with("#include");
        let mut cut = line.len();
        if let Some(pos) = line.find("//") {
            cut = cut.min(pos);
        }
        if !is_include {
            if let Some(pos) = line.find('#') {
                cut = cut.min(pos);
            }
        }
        out.push_str(&line[..cut]);
        if has_newline {
            out.push('\n');
        }
    }
    out
}
