use bf::io::Debug;
use bf::{brackets, io, ir, opt, parse, vm};
use std::fmt::Write as FmtWrite;
use std::io::sink;
use std::io::Cursor;

const TAPE_SIZE: usize = 8;
const TAPE_SNAPSHOT: usize = 128;
const MAX_STEPS: u64 = 100_000;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Termination {
    Ok,
    Underflow,
    MaxSteps,
    BracketError,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Outcome {
    output: Vec<u8>,
    pointer: usize,
    tape_prefix: Vec<u8>,
    termination: Termination,
}

fn classify_error(err: &str) -> Termination {
    if err.contains("pointer underflow") {
        Termination::Underflow
    } else if err.contains("max steps") {
        Termination::MaxSteps
    } else if err.contains("syntax error")
        || err.contains("unmatched '['")
        || err.contains("unmatched ']'")
    {
        Termination::BracketError
    } else {
        Termination::Other(err.to_string())
    }
}

fn blank_outcome(termination: Termination) -> Outcome {
    Outcome {
        output: Vec::new(),
        pointer: 0,
        tape_prefix: vec![0u8; TAPE_SNAPSHOT],
        termination,
    }
}

fn snapshot_tape(tape: &[u8]) -> Vec<u8> {
    let mut prefix = vec![0u8; TAPE_SNAPSHOT];
    let copy_len = std::cmp::min(TAPE_SNAPSHOT, tape.len());
    prefix[..copy_len].copy_from_slice(&tape[..copy_len]);
    prefix
}

fn bytes_hex(bytes: &[u8]) -> String {
    let mut out = String::new();
    for (idx, byte) in bytes.iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        let _ = write!(&mut out, "{:02x}", byte);
    }
    out
}

fn format_outcome(outcome: &Outcome) -> String {
    format!(
        "termination={:?} ptr={} output=[{}] tape[0..{}]=[{}]",
        outcome.termination,
        outcome.pointer,
        bytes_hex(&outcome.output),
        TAPE_SNAPSHOT,
        bytes_hex(&outcome.tape_prefix)
    )
}

fn compare_outcomes(
    case: &str,
    left_label: &str,
    left: &Outcome,
    right_label: &str,
    right: &Outcome,
    program: &str,
) {
    if left != right {
        panic!(
            "case '{}' mismatch between {} and {}\nprogram: {}\n{}: {}\n{}: {}",
            case,
            left_label,
            right_label,
            program,
            left_label,
            format_outcome(left),
            right_label,
            format_outcome(right)
        );
    }
}

fn build_ir(program: &str, optimize: bool) -> Result<Vec<ir::Instr>, String> {
    let ops = parse::filter_ops(program.as_bytes());
    let jumps = brackets::build_jumps(&ops)?;
    let mut ir = ir::build(&ops, &jumps)?;
    if optimize {
        opt::merge_ops(&mut ir);
        opt::rebuild_jumps(&mut ir)?;
        opt::loop_analysis(&mut ir);
        opt::peephole(&mut ir);
        opt::rebuild_jumps(&mut ir)?;
    }
    Ok(ir)
}

fn run_ir_pipeline(program: &str, optimize: bool) -> Outcome {
    let ir = match build_ir(program, optimize) {
        Ok(ir) => ir,
        Err(err) => return blank_outcome(classify_error(&err)),
    };

    let mut machine = vm::Vm::with_capacity(TAPE_SIZE).expect("tape size invalid");
    let mut input = io::Input::new(Cursor::new(Vec::new()));
    let mut output = io::Output::new(Vec::new());

    let mut dbg_sink = sink();
    let mut dbg = Debug::new(&mut dbg_sink);
    let result = machine.run_ir(&ir, &mut input, &mut output, Some(&mut dbg), Some(MAX_STEPS));
    let output_bytes = output.into_inner().expect("output buffer flush failed");
    let termination = match result {
        Ok(()) => Termination::Ok,
        Err(err) => classify_error(&err),
    };

    Outcome {
        output: output_bytes,
        pointer: machine.pointer(),
        tape_prefix: snapshot_tape(machine.tape()),
        termination,
    }
}

fn build_jumps(ops: &[u8]) -> Result<Vec<usize>, String> {
    let mut jumps = vec![usize::MAX; ops.len()];
    let mut stack = Vec::new();
    for (idx, &op) in ops.iter().enumerate() {
        match op {
            b'[' => stack.push(idx),
            b']' => {
                let open = stack
                    .pop()
                    .ok_or_else(|| format!("syntax error: unmatched ']' at {}", idx))?;
                jumps[open] = idx;
                jumps[idx] = open;
            }
            _ => {}
        }
    }
    if let Some(&open) = stack.last() {
        return Err(format!("syntax error: unmatched '[' at {}", open));
    }
    Ok(jumps)
}

fn ensure_capacity(tape: &mut Vec<u8>, required: usize) -> Result<(), String> {
    if required < tape.len() {
        return Ok(());
    }
    let mut new_len = tape.len().max(1);
    while new_len <= required {
        new_len = new_len
            .checked_mul(2)
            .ok_or_else(|| "runtime error: tape size overflow".to_string())?;
    }
    let additional = new_len
        .checked_sub(tape.len())
        .ok_or_else(|| "runtime error: tape size overflow".to_string())?;
    tape.try_reserve(additional)
        .map_err(|e| format!("runtime error: tape resize failed: {}", e))?;
    tape.resize(new_len, 0);
    Ok(())
}

fn run_reference(program: &str) -> Outcome {
    let ops = parse::filter_ops(program.as_bytes());
    let jumps = match build_jumps(&ops) {
        Ok(jumps) => jumps,
        Err(err) => return blank_outcome(classify_error(&err)),
    };

    let mut tape = vec![0u8; TAPE_SIZE];
    let mut pointer = 0usize;
    let mut ip = 0usize;
    let mut steps = 0u64;
    let mut output = Vec::new();
    let input = Vec::new();
    let mut input_idx = 0usize;

    let termination = loop {
        if ip >= ops.len() {
            break Termination::Ok;
        }
        if steps >= MAX_STEPS {
            break Termination::MaxSteps;
        }
        steps = match steps.checked_add(1) {
            Some(value) => value,
            None => break Termination::Other("runtime error: step counter overflow".to_string()),
        };

        match ops[ip] {
            b'+' => {
                tape[pointer] = tape[pointer].wrapping_add(1);
            }
            b'-' => {
                tape[pointer] = tape[pointer].wrapping_sub(1);
            }
            b'>' => {
                let next = match pointer.checked_add(1) {
                    Some(next) => next,
                    None => break Termination::Other("runtime error: pointer overflow".to_string()),
                };
                if let Err(err) = ensure_capacity(&mut tape, next) {
                    break Termination::Other(err);
                }
                pointer = next;
            }
            b'<' => {
                if pointer == 0 {
                    break Termination::Underflow;
                }
                pointer -= 1;
            }
            b'.' => output.push(tape[pointer]),
            b',' => {
                let value = if input_idx < input.len() {
                    let value = input[input_idx];
                    input_idx += 1;
                    value
                } else {
                    0
                };
                tape[pointer] = value;
            }
            b'[' => {
                if tape[pointer] == 0 {
                    let target = jumps[ip];
                    if target == usize::MAX {
                        break Termination::Other(
                            "runtime error: jump target out of range".to_string(),
                        );
                    }
                    ip = match target.checked_add(1) {
                        Some(next) => next,
                        None => {
                            break Termination::Other(
                                "runtime error: instruction pointer overflow".to_string(),
                            )
                        }
                    };
                    continue;
                }
            }
            b']' => {
                if tape[pointer] != 0 {
                    let target = jumps[ip];
                    if target == usize::MAX {
                        break Termination::Other(
                            "runtime error: jump target out of range".to_string(),
                        );
                    }
                    ip = target;
                    continue;
                }
            }
            _ => {}
        }

        ip = match ip.checked_add(1) {
            Some(next) => next,
            None => break Termination::Other("runtime error: instruction pointer overflow".to_string()),
        };
    };

    Outcome {
        output,
        pointer,
        tape_prefix: snapshot_tape(&tape),
        termination,
    }
}

fn assert_equivalence(case: &str, program: &str) {
    let reference = run_reference(program);
    let no_opt = run_ir_pipeline(program, false);
    let opt = run_ir_pipeline(program, true);

    compare_outcomes(case, "reference", &reference, "no-opt", &no_opt, program);
    compare_outcomes(case, "reference", &reference, "opt", &opt, program);
}

#[test]
fn equivalence_snippets() {
    let cases = [
        ("simple_output", "+++++."),
        ("clear_minus", "+[-]."),
        ("clear_plus", "+[+]."),
        ("scan_right", "+[>]"),
        ("underflow_left", "+[<]"),
        ("nested_loops", "++[>++[>+<-]<-]>>."),
        ("move_add_single", "+++[->+<]>."),
        ("move_add_multi", "+++[->+>+<<]>>."),
        ("move_add_multiplier", "+++[->++<]>."),
        ("move_add_wide", "+++[->>+<<]>>."),
        ("move_add_negative", ">+++[<+>-]<."),
        ("addmul_underflow", "+[<+>-]"),
        ("bracket_error", "+]"),
    ];

    for (case, program) in cases {
        assert_equivalence(case, program);
    }
}

#[test]
fn addmul_emitted_for_linear_loops() {
    let programs = ["+++[->+>+<<]>>.", "+++[->++<]>."];
    for program in programs {
        let ir = build_ir(program, true).expect("failed to build IR");
        let has_addmul = ir.iter().any(|instr| matches!(instr, ir::Instr::AddMul(_)));
        assert!(has_addmul, "expected AddMul in optimized IR for '{}'", program);
    }
}
