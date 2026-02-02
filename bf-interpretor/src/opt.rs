use crate::ir::Instr;

pub fn merge_ops(ir: &mut Vec<Instr>) {
    merge_linear(ir);
}

pub fn loop_analysis(ir: &mut Vec<Instr>) {
    let mut out = Vec::with_capacity(ir.len());
    let mut i = 0;
    while i < ir.len() {
        if i + 5 < ir.len() {
            if let Instr::Jz(target) = ir[i] {
                if target == i + 5 {
                    if let Instr::Add(dec) = ir[i + 1] {
                        if dec == -1 {
                            if let Instr::Move(offset) = ir[i + 2] {
                                if let Instr::Add(sign) = ir[i + 3] {
                                    if sign == 1 || sign == -1 {
                                        if let Instr::Move(back) = ir[i + 4] {
                                            if let Some(expected) = offset.checked_neg() {
                                                if back == expected {
                                                    if let Instr::Jnz(back_target) = ir[i + 5] {
                                                        if back_target == i {
                                                            out.push(Instr::AddTo(offset, sign));
                                                            i += 6;
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if i + 2 < ir.len() {
            if let Instr::Jz(target) = ir[i] {
                if target == i + 2 {
                    if let Instr::Move(dir) = ir[i + 1] {
                        if dir == 1 || dir == -1 {
                            if let Instr::Jnz(back) = ir[i + 2] {
                                if back == i {
                                    out.push(Instr::Scan(dir));
                                    i += 3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        if i + 2 < ir.len() {
            if let Instr::Jz(target) = ir[i] {
                if target == i + 2 {
                    if let Instr::Add(delta) = ir[i + 1] {
                        if delta == 1 || delta == -1 {
                            if let Instr::Jnz(back) = ir[i + 2] {
                                if back == i {
                                    out.push(Instr::SetZero);
                                    i += 3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        out.push(ir[i]);
        i += 1;
    }
    *ir = out;
}

pub fn peephole(ir: &mut Vec<Instr>) {
    merge_linear(ir);
}

pub fn rebuild_jumps(ir: &mut Vec<Instr>) -> Result<(), String> {
    let mut stack = Vec::new();
    for idx in 0..ir.len() {
        match ir[idx] {
            Instr::Jz(_) => stack.push(idx),
            Instr::Jnz(_) => {
                let open = stack
                    .pop()
                    .ok_or_else(|| format!("unmatched ']' at {}", idx))?;
                ir[open] = Instr::Jz(idx);
                ir[idx] = Instr::Jnz(open);
            }
            _ => {}
        }
    }

    if let Some(open) = stack.last() {
        return Err(format!("unmatched '[' at {}", open));
    }

    Ok(())
}

fn merge_linear(ir: &mut Vec<Instr>) {
    let mut out = Vec::with_capacity(ir.len());
    let mut i = 0;
    while i < ir.len() {
        match ir[i] {
            Instr::Add(_) => {
                let mut acc: i64 = 0;
                while i < ir.len() {
                    match ir[i] {
                        Instr::Add(delta) => {
                            acc += delta as i64;
                            i += 1;
                        }
                        _ => break,
                    }
                }
                push_add(&mut out, acc);
            }
            Instr::Move(_) => {
                let mut acc: i64 = 0;
                while i < ir.len() {
                    match ir[i] {
                        Instr::Move(delta) => {
                            acc += delta as i64;
                            i += 1;
                        }
                        _ => break,
                    }
                }
                push_move(&mut out, acc);
            }
            _ => {
                out.push(ir[i]);
                i += 1;
            }
        }
    }
    *ir = out;
}

fn push_add(out: &mut Vec<Instr>, mut acc: i64) {
    if acc == 0 {
        return;
    }
    while acc != 0 {
        let chunk = if acc > 0 {
            let max = i32::MAX as i64;
            if acc > max { max } else { acc }
        } else {
            let min = i32::MIN as i64;
            if acc < min { min } else { acc }
        };
        out.push(Instr::Add(chunk as i32));
        acc -= chunk;
    }
}

fn push_move(out: &mut Vec<Instr>, mut acc: i64) {
    if acc == 0 {
        return;
    }
    while acc != 0 {
        let chunk = if acc > 0 {
            let max = i32::MAX as i64;
            if acc > max { max } else { acc }
        } else {
            let min = i32::MIN as i64;
            if acc < min { min } else { acc }
        };
        out.push(Instr::Move(chunk as i32));
        acc -= chunk;
    }
}
