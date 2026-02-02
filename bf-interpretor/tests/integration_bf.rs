use std::env;
use std::path::PathBuf;
use std::process::Command;

fn bf_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("bf");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

fn run_bf(args: &[&str]) -> std::process::Output {
    Command::new(bf_path())
        .args(args)
        .output()
        .expect("failed to run bf")
}

#[test]
fn pure_a_outputs_byte() {
    let out = run_bf(&["programs/tests/pure_A.bf"]);
    assert!(out.status.success());
    assert_eq!(out.stdout, vec![0x41]);
    assert!(out.stderr.is_empty());
}

#[test]
fn loop_small_outputs_known_byte() {
    let out = run_bf(&["programs/tests/loop_small.bf"]);
    assert!(out.status.success());
    assert_eq!(out.stdout, b"B");
    assert!(out.stderr.is_empty());
}

#[test]
fn memory_walk_outputs_ab() {
    let out = run_bf(&["programs/stress/memory_walk.bf"]);
    assert!(out.status.success());
    assert_eq!(out.stdout, b"AB");
    assert!(out.stderr.is_empty());
}

#[test]
fn max_steps_exits_nonzero() {
    let out = run_bf(&["--max-steps", "1", "programs/stress/loop_heavy.bf"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("max steps"));
}
