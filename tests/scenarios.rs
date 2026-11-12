//! The acceptance scenarios of the `peppec-test median` command line
//! (specification of #60). Each test runs the real binary as a process and
//! checks its standard output, standard error and exit code.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .args(args)
        .output()
        .unwrap()
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8(out.stderr.clone()).unwrap()
}

/// Asserts the exact standard output, standard error and exit code.
fn assert_exact(args: &[&str], stdout: &str, stderr: &str, code: i32) {
    let out = run(args);
    assert_eq!(stdout_of(&out), stdout, "standard output");
    assert_eq!(stderr_of(&out), stderr, "standard error");
    assert_eq!(out.status.code(), Some(code), "exit code");
}

#[test]
fn odd_count() {
    assert_exact(&["median", "3", "1", "2"], "2\n", "", 0);
}

#[test]
fn even_count() {
    assert_exact(&["median", "4", "1", "3", "2"], "2.5\n", "", 0);
}

#[test]
fn single_number() {
    assert_exact(&["median", "7"], "7\n", "", 0);
}

#[test]
fn negative_numbers() {
    assert_exact(&["median", "-5", "-1", "-3"], "-3\n", "", 0);
}

#[test]
fn explicit_plus_sign() {
    assert_exact(&["median", "+4", "+2"], "3\n", "", 0);
}

#[test]
fn extremes_of_the_32_bit_range() {
    assert_exact(&["median", "-2147483648", "2147483647"], "-0.5\n", "", 0);
}

#[test]
fn unknown_command() {
    let out = run(&["avg", "1", "2"]);
    assert_eq!(stdout_of(&out), "", "standard output");
    let stderr = stderr_of(&out);
    assert_eq!(stderr.lines().count(), 1, "standard error is one line");
    assert!(
        stderr.starts_with("error: unknown command 'avg'"),
        "standard error: {stderr:?}"
    );
    assert!(stderr.contains("median"), "standard error: {stderr:?}");
    assert_eq!(out.status.code(), Some(2), "exit code");
}

#[test]
fn invalid_number() {
    assert_exact(
        &["median", "1", "abc", "3"],
        "",
        "error: 'abc' is not a valid number\n",
        2,
    );
}

#[test]
fn decimal_number() {
    assert_exact(
        &["median", "1.5", "2"],
        "",
        "error: '1.5' is not a valid number\n",
        2,
    );
}

#[test]
fn number_out_of_range() {
    assert_exact(
        &["median", "1", "2147483648"],
        "",
        "error: '2147483648' is out of range for a 32-bit integer\n",
        2,
    );
}

#[test]
fn only_the_first_invalid_value_is_reported() {
    let out = run(&["median", "x", "y"]);
    assert_eq!(stdout_of(&out), "", "standard output");
    let stderr = stderr_of(&out);
    assert_eq!(
        stderr, "error: 'x' is not a valid number\n",
        "standard error"
    );
    assert!(
        !stderr.contains("'y'"),
        "standard error mentions y: {stderr:?}"
    );
    assert_eq!(out.status.code(), Some(2), "exit code");
}

#[test]
fn no_numbers() {
    assert_exact(
        &["median"],
        "",
        "error: median needs at least one number\n",
        2,
    );
}

#[test]
fn no_arguments() {
    let out = run(&[]);
    assert_eq!(stdout_of(&out), "", "standard output");
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("usage: peppec-test <command> [numbers...]"),
        "standard error: {stderr:?}"
    );
    assert_eq!(out.status.code(), Some(2), "exit code");
}

#[test]
fn help() {
    let out = run(&["--help"]);
    let stdout = stdout_of(&out);
    assert!(
        stdout.contains("usage: peppec-test <command> [numbers...]"),
        "standard output: {stdout:?}"
    );
    assert!(stdout.contains("median"), "standard output: {stdout:?}");
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}
