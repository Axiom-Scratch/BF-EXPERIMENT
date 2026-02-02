use bf::io;
use bf::io::Debug;
use bf::ir::Instr;
use bf::vm::Vm;
use std::io::sink;
use std::io::Cursor;

#[test]
fn outputs_byte() {
    let ir = vec![Instr::Add(65), Instr::Output];
    let mut machine = Vm::with_capacity(1).unwrap();
    let mut input = io::Input::new(Cursor::new(Vec::new()));
    let mut output = io::Output::new(Vec::new());
    let mut dbg_sink = sink();
    let mut dbg = Debug::new(&mut dbg_sink);
    machine
        .run_ir(&ir, &mut input, &mut output, Some(&mut dbg), None)
        .unwrap();
    let out = output.into_inner().unwrap();
    assert_eq!(out, vec![65]);
}

#[test]
fn echoes_input() {
    let ir = vec![Instr::Input, Instr::Output];
    let mut machine = Vm::with_capacity(1).unwrap();
    let mut input = io::Input::new(Cursor::new(vec![0x41]));
    let mut output = io::Output::new(Vec::new());
    let mut dbg_sink = sink();
    let mut dbg = Debug::new(&mut dbg_sink);
    machine
        .run_ir(&ir, &mut input, &mut output, Some(&mut dbg), None)
        .unwrap();
    let out = output.into_inner().unwrap();
    assert_eq!(out, vec![0x41]);
}

#[test]
fn addto_updates_target() {
    let ir = vec![
        Instr::Add(2),
        Instr::AddTo(1, 1),
        Instr::Move(1),
        Instr::Output,
    ];
    let mut machine = Vm::with_capacity(2).unwrap();
    let mut input = io::Input::new(Cursor::new(Vec::new()));
    let mut output = io::Output::new(Vec::new());
    let mut dbg_sink = sink();
    let mut dbg = Debug::new(&mut dbg_sink);
    machine
        .run_ir(&ir, &mut input, &mut output, Some(&mut dbg), None)
        .unwrap();
    let out = output.into_inner().unwrap();
    assert_eq!(out, vec![2]);
}

#[test]
fn grows_tape_on_move() {
    let ir = vec![Instr::Move(10)];
    let mut machine = Vm::with_capacity(1).unwrap();
    let mut input = io::Input::new(Cursor::new(Vec::new()));
    let mut output = io::Output::new(Vec::new());
    let mut dbg_sink = sink();
    let mut dbg = Debug::new(&mut dbg_sink);
    machine
        .run_ir(&ir, &mut input, &mut output, Some(&mut dbg), None)
        .unwrap();
    assert_eq!(machine.pointer(), 10);
    assert!(machine.tape().len() > 10);
}
