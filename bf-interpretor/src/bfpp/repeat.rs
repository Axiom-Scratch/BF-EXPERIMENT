use super::error::Error;

pub fn expand_repeats(text: &str) -> Result<String, Error> {
    let (out, _) = parse_section(text, 0, None)?;
    Ok(out)
}

fn parse_section(text: &str, mut i: usize, end_char: Option<u8>) -> Result<(String, usize), Error> {
    let bytes = text.as_bytes();
    let mut out = String::new();
    while i < bytes.len() {
        if let Some(end) = end_char {
            if bytes[i] == end {
                return Ok((out, i + 1));
            }
        }
        if bytes[i] == b'@' && text[i..].starts_with("@repeat") {
            let start = i;
            i += "@repeat".len();
            let (next_i, had_ws) = skip_ws(text, i);
            if !had_ws {
                return Err(Error::RepeatError {
                    message: format!("invalid @repeat syntax at byte {}", start),
                });
            }
            i = next_i;
            let (count, next_i) = parse_number(text, i)?;
            if count == 0 {
                return Err(Error::RepeatError {
                    message: format!("repeat count must be positive at byte {}", i),
                });
            }
            i = next_i;
            let (next_i, had_ws) = skip_ws(text, i);
            if !had_ws {
                return Err(Error::RepeatError {
                    message: format!("invalid @repeat syntax at byte {}", start),
                });
            }
            i = next_i;
            if i >= bytes.len() {
                return Err(Error::RepeatError {
                    message: format!("missing repeat target at byte {}", start),
                });
            }
            if bytes[i] == b'{' {
                let (inner, next_i) = parse_section(text, i + 1, Some(b'}'))?;
                for _ in 0..count {
                    out.push_str(&inner);
                }
                i = next_i;
            } else {
                let (ch, len) = next_char(text, i)?;
                if ch.is_whitespace() {
                    return Err(Error::RepeatError {
                        message: format!("invalid repeat target at byte {}", i),
                    });
                }
                for _ in 0..count {
                    out.push(ch);
                }
                i += len;
            }
            continue;
        }
        let (ch, len) = next_char(text, i)?;
        out.push(ch);
        i += len;
    }
    if end_char.is_some() {
        Err(Error::RepeatError {
            message: "missing '}'".to_string(),
        })
    } else {
        Ok((out, i))
    }
}

fn skip_ws(text: &str, mut i: usize) -> (usize, bool) {
    let bytes = text.as_bytes();
    let mut consumed = false;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        consumed = true;
        i += 1;
    }
    (i, consumed)
}

fn parse_number(text: &str, start: usize) -> Result<(usize, usize), Error> {
    let bytes = text.as_bytes();
    let mut i = start;
    let mut value: usize = 0;
    let mut saw = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            saw = true;
            let digit = (b - b'0') as usize;
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(digit))
                .ok_or_else(|| Error::RepeatError {
                    message: "repeat count overflow".to_string(),
                })?;
            i += 1;
        } else {
            break;
        }
    }
    if !saw {
        return Err(Error::RepeatError {
            message: format!("expected repeat count at byte {}", start),
        });
    }
    Ok((value, i))
}

fn next_char(text: &str, i: usize) -> Result<(char, usize), Error> {
    text[i..]
        .chars()
        .next()
        .map(|ch| (ch, ch.len_utf8()))
        .ok_or_else(|| Error::RepeatError {
            message: "unexpected end of input".to_string(),
        })
}
