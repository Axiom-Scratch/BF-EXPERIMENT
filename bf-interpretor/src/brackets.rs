pub fn build_jumps(ops: &[u8]) -> Result<Vec<usize>, String> {
    let mut jumps = vec![usize::MAX; ops.len()];
    let mut stack = Vec::new();

    for (idx, &op) in ops.iter().enumerate() {
        match op {
            b'[' => stack.push(idx),
            b']' => {
                let open = stack
                    .pop()
                    .ok_or_else(|| format!("unmatched ']' at {}", idx))?;
                jumps[open] = idx;
                jumps[idx] = open;
            }
            _ => {}
        }
    }

    if let Some(&open) = stack.last() {
        return Err(format!("unmatched '[' at {}", open));
    }

    Ok(jumps)
}
