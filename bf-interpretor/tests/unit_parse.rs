use bf::parse::filter_ops;

#[test]
fn filters_only_ops() {
    let input = b"ab+<>-.,[]xyz\n";
    let output = filter_ops(input);
    assert_eq!(output, b"+<>-.,[]");
}
