use cli_test_dir::*;

const BIN: &'static str = "./main";

fn test_function(input: &str, expected: &str) -> () {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), expected);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample1() {
    let input = r#"2
    3 1 2
    6 1 1
    "#;

    let expected = "0\n";
    test_function(input, expected);
}

#[test]
fn sample2() {
    let input = r#"2
    3 1 2
    6 1 1
    "#;

    let expected = "0\n";
    test_function(input, expected);
}

#[test]
fn sample3() {
    let input = r#"2
    3 1 2
    6 1 1
    "#;

    let expected = "0\n";
    test_function(input, expected);
}
