use std::path::Path;

use super::error::Error;
use super::include;

pub fn preprocess(input_path: &Path) -> Result<String, Error> {
    let bytes = std::fs::read(input_path)
        .map_err(|e| Error::ReadFailed { path: input_path.to_path_buf(), source: e })?;
    let text = String::from_utf8(bytes)
        .map_err(|_| Error::InvalidUtf8 { path: input_path.to_path_buf() })?;
    let base_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let resolved = include::resolve_includes(&text, base_dir)?;
    Ok(resolved)
}
