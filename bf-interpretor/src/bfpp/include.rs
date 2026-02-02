use std::path::{Path, PathBuf};

use super::error::Error;

pub fn resolve_includes(input_path: &Path) -> Result<String, Error> {
    let mut stack = Vec::new();
    resolve_path(input_path, &mut stack)
}

fn resolve_path(path: &Path, stack: &mut Vec<PathBuf>) -> Result<String, Error> {
    let canonical = std::fs::canonicalize(path)
        .map_err(|e| Error::ReadFailed { path: path.to_path_buf(), source: e })?;
    if let Some(pos) = stack.iter().position(|p| p == &canonical) {
        let mut cycle = String::new();
        for (i, p) in stack[pos..].iter().enumerate() {
            if i > 0 {
                cycle.push_str(" -> ");
            }
            cycle.push_str(&p.display().to_string());
        }
        if !cycle.is_empty() {
            cycle.push_str(" -> ");
        }
        cycle.push_str(&canonical.display().to_string());
        return Err(Error::IncludeError {
            path: path.to_path_buf(),
            message: format!("include cycle: {}", cycle),
        });
    }
    stack.push(canonical);

    let bytes = std::fs::read(path)
        .map_err(|e| Error::ReadFailed { path: path.to_path_buf(), source: e })?;
    let text = String::from_utf8(bytes)
        .map_err(|_| Error::InvalidUtf8 { path: path.to_path_buf() })?;

    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut out = String::new();

    for part in text.split_inclusive('\n') {
        let has_newline = part.ends_with('\n');
        let mut line = &part[..part.len() - if has_newline { 1 } else { 0 }];
        if line.ends_with('\r') {
            line = &line[..line.len() - 1];
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with("#include") {
            let include_path = parse_include_path(trimmed, path)?;
            if include_path.is_absolute() {
                return Err(Error::IncludeError {
                    path: path.to_path_buf(),
                    message: "include path must be relative".to_string(),
                });
            }
            let full_path = base_dir.join(include_path);
            let resolved = resolve_path(&full_path, stack)?;
            out.push_str(&resolved);
            if has_newline && !resolved.ends_with('\n') {
                out.push('\n');
            }
        } else {
            out.push_str(line);
            if has_newline {
                out.push('\n');
            }
        }
    }

    stack.pop();
    Ok(out)
}

fn parse_include_path(line: &str, path: &Path) -> Result<PathBuf, Error> {
    let rest = &line["#include".len()..];
    let rest = rest.trim_start();
    if !rest.starts_with('"') {
        return Err(Error::IncludeError {
            path: path.to_path_buf(),
            message: "invalid include syntax".to_string(),
        });
    }
    let rest = &rest[1..];
    let end = match rest.find('"') {
        Some(value) => value,
        None => {
            return Err(Error::IncludeError {
                path: path.to_path_buf(),
                message: "invalid include syntax".to_string(),
            })
        }
    };
    let path_str = &rest[..end];
    if path_str.is_empty() {
        return Err(Error::IncludeError {
            path: path.to_path_buf(),
            message: "invalid include syntax".to_string(),
        });
    }
    let tail = &rest[end + 1..];
    let tail_trim = tail.trim_start();
    if !tail_trim.is_empty() && !tail_trim.starts_with("//") && !tail_trim.starts_with('#') {
        return Err(Error::IncludeError {
            path: path.to_path_buf(),
            message: "invalid include syntax".to_string(),
        });
    }
    Ok(PathBuf::from(path_str))
}
