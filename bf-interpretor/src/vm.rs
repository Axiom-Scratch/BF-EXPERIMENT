use crate::io::{Debug, Input, Output};
use crate::ir::Instr;
use std::io::{Read, Write};

pub struct Vm {
    tape: Vec<u8>,
    pointer: usize,
}

impl Vm {
    pub fn with_capacity(tape_size: usize) -> Result<Self, String> {
        if tape_size == 0 {
            return Err("tape size must be greater than 0".to_string());
        }
        let mut tape = Vec::with_capacity(tape_size);
        tape.resize(tape_size, 0);
        Ok(Self { tape, pointer: 0 })
    }

    fn ensure_capacity(&mut self, required: usize) -> Result<(), String> {
        if required < self.tape.len() {
            return Ok(());
        }

        let mut new_len = self.tape.len().max(1);
        while new_len <= required {
            new_len = new_len
                .checked_mul(2)
                .ok_or_else(|| "runtime error: tape size overflow".to_string())?;
        }

        let additional = new_len
            .checked_sub(self.tape.len())
            .ok_or_else(|| "runtime error: tape size overflow".to_string())?;
        self.tape
            .try_reserve(additional)
            .map_err(|e| format!("runtime error: tape resize failed: {}", e))?;
        self.tape.resize(new_len, 0);
        Ok(())
    }

    fn move_ptr(&mut self, delta: i32) -> Result<(), String> {
        if delta == 0 {
            return Ok(());
        }
        let delta_i64 = delta as i64;
        if delta_i64 > 0 {
            let shift = delta_i64 as usize;
            let next = self
                .pointer
                .checked_add(shift)
                .ok_or_else(|| "runtime error: pointer overflow".to_string())?;
            if next >= self.tape.len() {
                self.ensure_capacity(next)?;
            }
            self.pointer = next;
        } else {
            let shift = (-delta_i64) as usize;
            if shift > self.pointer {
                return Err("runtime error: pointer underflow".to_string());
            }
            self.pointer -= shift;
        }
        Ok(())
    }

    fn offset_index(&mut self, offset: i32) -> Result<usize, String> {
        if offset == 0 {
            return Ok(self.pointer);
        }
        if offset > 0 {
            let shift = offset as usize;
            let target = self
                .pointer
                .checked_add(shift)
                .ok_or_else(|| "runtime error: pointer overflow".to_string())?;
            if target >= self.tape.len() {
                self.ensure_capacity(target)?;
            }
            Ok(target)
        } else {
            let shift = (-offset) as usize;
            if shift > self.pointer {
                return Err("runtime error: pointer underflow".to_string());
            }
            Ok(self.pointer - shift)
        }
    }

    fn add_cell(&mut self, delta: i32) {
        let value = self.tape[self.pointer];
        self.tape[self.pointer] = value.wrapping_add(delta as u8);
    }

    pub fn run_ir<R, W, E>(
        &mut self,
        ir: &[Instr],
        input: &mut Input<R>,
        output: &mut Output<W>,
        debug: Option<&mut Debug<E>>,
        max_steps: Option<u64>,
    ) -> Result<(), String>
    where
        R: Read,
        W: Write,
        E: Write,
    {
        self.run(ir, input, output, debug, max_steps)
    }

    pub fn run<R, W, E>(
        &mut self,
        ir: &[Instr],
        input: &mut Input<R>,
        output: &mut Output<W>,
        mut debug: Option<&mut Debug<E>>,
        max_steps: Option<u64>,
    ) -> Result<(), String>
    where
        R: Read,
        W: Write,
        E: Write,
    {
        let mut ip = 0usize;
        let mut steps = 0u64;

        while ip < ir.len() {
            if let Some(limit) = max_steps {
                if steps >= limit {
                    return Err("runtime error: max steps exceeded".to_string());
                }
            }

            let instr = ir[ip];
            if let Some(ref mut debug) = debug {
                self.trace(debug, steps, ip, instr)?;
            }

            steps = steps
                .checked_add(1)
                .ok_or_else(|| "runtime error: step counter overflow".to_string())?;

            match instr {
                Instr::Add(delta) => {
                    self.add_cell(delta);
                }
                Instr::Move(delta) => {
                    self.move_ptr(delta)?;
                }
                Instr::AddTo(offset, sign) => {
                    let value = self.tape[self.pointer];
                    if value != 0 {
                        let target = self.offset_index(offset)?;
                        let dest = self.tape[target];
                        let updated = if sign < 0 {
                            dest.wrapping_sub(value)
                        } else {
                            dest.wrapping_add(value)
                        };
                        self.tape[target] = updated;
                        self.tape[self.pointer] = 0;
                    }
                }
                Instr::Output => {
                    let byte = self.tape[self.pointer];
                    output.write_byte(byte)?;
                }
                Instr::Input => {
                    let value = input.read_byte()?;
                    self.tape[self.pointer] = value;
                }
                Instr::SetZero => {
                    self.tape[self.pointer] = 0;
                }
                Instr::Scan(dir) => {
                    if dir == 0 {
                        return Err("runtime error: scan direction zero".to_string());
                    }
                    let step = if dir > 0 { 1 } else { -1 };
                    while self.tape[self.pointer] != 0 {
                        self.move_ptr(step)?;
                    }
                }
                Instr::Jz(target) => {
                    if self.tape[self.pointer] == 0 {
                        if target >= ir.len() {
                            return Err("runtime error: jump target out of range".to_string());
                        }
                        ip = target
                            .checked_add(1)
                            .ok_or_else(|| "runtime error: instruction pointer overflow".to_string())?;
                        continue;
                    }
                }
                Instr::Jnz(target) => {
                    if self.tape[self.pointer] != 0 {
                        if target >= ir.len() {
                            return Err("runtime error: jump target out of range".to_string());
                        }
                        ip = target;
                        continue;
                    }
                }
            }

            ip = ip
                .checked_add(1)
                .ok_or_else(|| "runtime error: instruction pointer overflow".to_string())?;
        }

        output.flush()?;
        Ok(())
    }

    fn trace<E: Write>(
        &self,
        debug: &mut Debug<E>,
        steps: u64,
        ip: usize,
        instr: Instr,
    ) -> Result<(), String> {
        let cell = self.tape[self.pointer];
        debug.write_fmt(format_args!(
            "step={} ip={} ptr={} cell={} ",
            steps, ip, self.pointer, cell
        ))?;
        match instr {
            Instr::Add(delta) => debug.write_fmt(format_args!("Add {}\n", delta)),
            Instr::Move(delta) => debug.write_fmt(format_args!("Move {}\n", delta)),
            Instr::AddTo(offset, sign) => {
                debug.write_fmt(format_args!("AddTo {} {}\n", offset, sign))
            }
            Instr::Output => debug.write_fmt(format_args!("Output\n")),
            Instr::Input => debug.write_fmt(format_args!("Input\n")),
            Instr::Jz(target) => debug.write_fmt(format_args!("Jz {}\n", target)),
            Instr::Jnz(target) => debug.write_fmt(format_args!("Jnz {}\n", target)),
            Instr::SetZero => debug.write_fmt(format_args!("SetZero\n")),
            Instr::Scan(dir) => debug.write_fmt(format_args!("Scan {}\n", dir)),
        }
    }
}
