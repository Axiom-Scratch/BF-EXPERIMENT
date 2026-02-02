use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    Add(i32),
    Move(i32),
    AddTo(i32, i32),
    AddMul(Vec<(i32, i32)>),
    Output,
    Input,
    Jz(usize),
    Jnz(usize),
    SetZero,
    Scan(i32),
}

pub fn build(ops: &[u8], jumps: &[usize]) -> Result<Vec<Instr>, String> {
    if ops.len() != jumps.len() {
        return Err("ops and jumps length mismatch".to_string());
    }

    let mut ir = Vec::with_capacity(ops.len());
    for (idx, &op) in ops.iter().enumerate() {
        match op {
            b'+' => ir.push(Instr::Add(1)),
            b'-' => ir.push(Instr::Add(-1)),
            b'>' => ir.push(Instr::Move(1)),
            b'<' => ir.push(Instr::Move(-1)),
            b'.' => ir.push(Instr::Output),
            b',' => ir.push(Instr::Input),
            b'[' => {
                let target = jumps[idx];
                if target == usize::MAX {
                    return Err(format!("missing jump target for '[' at {}", idx));
                }
                ir.push(Instr::Jz(target));
            }
            b']' => {
                let target = jumps[idx];
                if target == usize::MAX {
                    return Err(format!("missing jump target for ']' at {}", idx));
                }
                ir.push(Instr::Jnz(target));
            }
            _ => {}
        }
    }
    Ok(ir)
}

pub fn dump_ir<W: Write>(ir: &[Instr], out: &mut W) -> Result<(), String> {
    for (idx, instr) in ir.iter().enumerate() {
        let result: Result<(), std::io::Error> = match instr {
            Instr::Add(delta) => writeln!(out, "{} Add {}", idx, delta),
            Instr::Move(delta) => writeln!(out, "{} Move {}", idx, delta),
            Instr::AddTo(offset, sign) => writeln!(out, "{} AddTo {} {}", idx, offset, sign),
            Instr::AddMul(edits) => (|| {
                write!(out, "{} AddMul", idx)?;
                for (offset, factor) in edits {
                    write!(out, " ({},{})", offset, factor)?;
                }
                writeln!(out)
            })(),
            Instr::Output => writeln!(out, "{} Output", idx),
            Instr::Input => writeln!(out, "{} Input", idx),
            Instr::Jz(target) => writeln!(out, "{} Jz {}", idx, target),
            Instr::Jnz(target) => writeln!(out, "{} Jnz {}", idx, target),
            Instr::SetZero => writeln!(out, "{} SetZero", idx),
            Instr::Scan(dir) => writeln!(out, "{} Scan {}", idx, dir),
        };
        result.map_err(|e| format!("stderr write failed: {}", e))?;
    }
    Ok(())
}
