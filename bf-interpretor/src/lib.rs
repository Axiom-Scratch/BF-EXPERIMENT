pub mod brackets;
pub mod parse;
pub mod vm;

pub fn run_file(path: &str) -> Result<(), String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("failed to read '{}': {}", path, e))?;
    let ops = parse::filter_ops(&bytes);
    let jumps = brackets::build_jumps(&ops)?;
    let mut machine = vm::Vm::new();
    machine.run(&ops, &jumps)
}
