use std::io::{self, BufReader, BufWriter, Read, Write};

const TAPE_SIZE: usize = 30_000;

pub struct Vm {
    tape: Vec<u8>,
    pointer: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            tape: vec![0; TAPE_SIZE],
            pointer: 0,
        }
    }

    pub fn run(&mut self, ops: &[u8], jumps: &[usize]) -> Result<(), String> {
        if ops.len() != jumps.len() {
            return Err("ops and jumps length mismatch".to_string());
        }

        let stdin = io::stdin();
        let mut stdin = BufReader::new(stdin.lock());
        let stdout = io::stdout();
        let mut stdout = BufWriter::new(stdout.lock());
        let mut read_buf = [0u8; 1];
        let mut ip = 0usize;

        while ip < ops.len() {
            match ops[ip] {
                b'>' => {
                    let next = self
                        .pointer
                        .checked_add(1)
                        .ok_or_else(|| "pointer overflow".to_string())?;
                    if next >= self.tape.len() {
                        return Err("pointer overflow".to_string());
                    }
                    self.pointer = next;
                }
                b'<' => {
                    if self.pointer == 0 {
                        return Err("pointer underflow".to_string());
                    }
                    self.pointer -= 1;
                }
                b'+' => {
                    let value = self.tape[self.pointer];
                    self.tape[self.pointer] = value.wrapping_add(1);
                }
                b'-' => {
                    let value = self.tape[self.pointer];
                    self.tape[self.pointer] = value.wrapping_sub(1);
                }
                b'.' => {
                    let byte = [self.tape[self.pointer]];
                    stdout
                        .write_all(&byte)
                        .map_err(|e| format!("stdout write failed: {}", e))?;
                }
                b',' => {
                    let read = stdin
                        .read(&mut read_buf)
                        .map_err(|e| format!("stdin read failed: {}", e))?;
                    let value = if read == 0 { 0 } else { read_buf[0] };
                    self.tape[self.pointer] = value;
                }
                b'[' => {
                    if self.tape[self.pointer] == 0 {
                        let target = jumps[ip];
                        if target == usize::MAX {
                            return Err("missing jump target for '['".to_string());
                        }
                        ip = target
                            .checked_add(1)
                            .ok_or_else(|| "instruction pointer overflow".to_string())?;
                        continue;
                    }
                }
                b']' => {
                    if self.tape[self.pointer] != 0 {
                        let target = jumps[ip];
                        if target == usize::MAX {
                            return Err("missing jump target for ']'".to_string());
                        }
                        ip = target;
                        continue;
                    }
                }
                _ => {}
            }

            ip = ip
                .checked_add(1)
                .ok_or_else(|| "instruction pointer overflow".to_string())?;
        }

        stdout
            .flush()
            .map_err(|e| format!("stdout flush failed: {}", e))?;
        Ok(())
    }
}
