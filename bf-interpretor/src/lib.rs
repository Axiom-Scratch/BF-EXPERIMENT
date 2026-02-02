pub mod brackets;
pub mod io;
pub mod ir;
pub mod opt;
pub mod parse;
pub mod vm;

pub fn run_file(
    path: &str,
    tape_size: usize,
    max_steps: Option<u64>,
    dump_ir: bool,
    trace: bool,
) -> Result<(), String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("failed to read '{}': {}", path, e))?;
    let ops = parse::filter_ops(&bytes);
    let jumps = brackets::build_jumps(&ops)?;
    let mut ir = ir::build(&ops, &jumps)?;
    opt::merge_ops(&mut ir);
    opt::rebuild_jumps(&mut ir)?;
    opt::loop_analysis(&mut ir);
    opt::peephole(&mut ir);
    opt::rebuild_jumps(&mut ir)?;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut input = io::Input::new(stdin.lock());
    let mut output = io::Output::new(stdout.lock());
    let mut debug = io::Debug::new(stderr.lock());

    if dump_ir {
        ir::dump_ir(&ir, debug.writer())?;
        debug.flush()?;
    }

    let mut machine = vm::Vm::with_capacity(tape_size)?;
    vm::Vm::run_ir(
        &mut machine,
        &ir,
        &mut input,
        &mut output,
        if trace { Some(&mut debug) } else { None },
        max_steps,
    )?;

    if trace {
        debug.flush()?;
    }

    Ok(())
}
