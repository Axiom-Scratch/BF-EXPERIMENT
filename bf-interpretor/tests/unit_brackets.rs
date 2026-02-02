use bf::brackets::build_jumps;

#[test]
fn builds_nested_jumps() {
    let ops = b"[[]]";
    let jumps = build_jumps(ops).unwrap();
    assert_eq!(jumps, vec![3, 2, 1, 0]);
}

#[test]
fn errors_on_unmatched_right() {
    let ops = b"]";
    let err = build_jumps(ops).unwrap_err();
    assert_eq!(err, "syntax error: unmatched ']' at 0");
}

#[test]
fn errors_on_unmatched_left() {
    let ops = b"[";
    let err = build_jumps(ops).unwrap_err();
    assert_eq!(err, "syntax error: unmatched '[' at 0");
}
