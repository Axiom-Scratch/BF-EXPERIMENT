use bf::{brackets, ir, parse};

#[test]
fn builds_ir_for_loop() {
    let ops = parse::filter_ops(b"[+]");
    let jumps = brackets::build_jumps(&ops).unwrap();
    let ir = ir::build(&ops, &jumps).unwrap();
    assert_eq!(
        ir,
        vec![ir::Instr::Jz(2), ir::Instr::Add(1), ir::Instr::Jnz(0)]
    );
}

#[test]
fn errors_on_length_mismatch() {
    let ops = vec![b'+'];
    let jumps = vec![];
    let err = ir::build(&ops, &jumps).unwrap_err();
    assert_eq!(err, "ops and jumps length mismatch");
}
