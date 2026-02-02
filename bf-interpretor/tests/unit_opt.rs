use bf::ir::Instr;
use bf::opt;

#[test]
fn merges_add_and_move() {
    let mut ir = vec![Instr::Add(1), Instr::Add(2), Instr::Move(1), Instr::Move(2)];
    opt::merge_ops(&mut ir);
    assert_eq!(ir, vec![Instr::Add(3), Instr::Move(3)]);
}

#[test]
fn removes_no_ops() {
    let mut ir = vec![
        Instr::Add(1),
        Instr::Add(-1),
        Instr::Move(1),
        Instr::Move(-1),
        Instr::Output,
    ];
    opt::merge_ops(&mut ir);
    assert_eq!(ir, vec![Instr::Output]);
}

#[test]
fn detects_setzero() {
    let mut ir = vec![Instr::Jz(2), Instr::Add(-1), Instr::Jnz(0)];
    opt::loop_analysis(&mut ir);
    assert_eq!(ir, vec![Instr::SetZero]);
}
