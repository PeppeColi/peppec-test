use std::process::Command;

fn check(args: &[&str], expected: &str) {
    let out = Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .args(args)
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        expected.to_owned() + "\n"
    );
    assert!(out.stderr.is_empty());
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn median_odd_count() {
    check(&["median", "3", "1", "2"], "2");
}

#[test]
fn median_even_count() {
    check(&["median", "4", "1", "3", "2"], "2.5");
}

#[test]
fn median_single_value() {
    check(&["median", "7"], "7");
}

#[test]
fn median_negative_values() {
    check(&["median", "-5", "-1", "-3"], "-3");
}

#[test]
fn median_plus_sign() {
    check(&["median", "+4", "+2"], "3");
}

#[test]
fn median_at_extremes() {
    check(&["median", "-2147483648", "2147483647"], "-0.5");
}

fn check_error(args: &[&str], expected_error: &str) {
    let out = Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .args(args)
        .output()
        .unwrap();
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        expected_error.to_owned() + "\n"
    );
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn median_invalid_number() {
    check_error(
        &["median", "1", "abc", "3"],
        "error: 'abc' is not a valid number",
    );
}

#[test]
fn median_decimal_number() {
    check_error(
        &["median", "1.5", "2"],
        "error: '1.5' is not a valid number",
    );
}

#[test]
fn median_number_out_of_range() {
    check_error(
        &["median", "1", "2147483648"],
        "error: '2147483648' is out of range for a 32-bit integer",
    );
}

#[test]
fn median_reports_only_first_invalid_value() {
    check_error(&["median", "x", "y"], "error: 'x' is not a valid number");
}

#[test]
fn median_without_numbers() {
    check_error(&["median"], "error: median needs at least one number");
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn ping_prints_pong() {
    check(&["ping"], "pong");
}

#[test]
fn unknown_command() {
    let out = run(&["avg", "1", "2"]);
    assert!(out.stdout.is_empty());
    let err = String::from_utf8(out.stderr).unwrap();
    assert_eq!(err.lines().count(), 1);
    assert!(err.starts_with("error: unknown command 'avg'"));
    assert!(err.contains("median"));
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn no_arguments_prints_usage_to_error_stream() {
    let out = run(&[]);
    assert!(out.stdout.is_empty());
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.contains("usage: peppec-test <command> [numbers...]"));
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn help_prints_usage_to_output_stream() {
    let out = run(&["--help"]);
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("usage: peppec-test <command> [numbers...]"));
    assert!(stdout.contains("median"));
    assert!(out.stderr.is_empty());
    assert_eq!(out.status.code(), Some(0));
}
