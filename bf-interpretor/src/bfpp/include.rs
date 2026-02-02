use std::path::Path;

use super::error::Error;

pub fn resolve_includes(text: &str, _base_dir: &Path) -> Result<String, Error> {
    Ok(text.to_string())
}
