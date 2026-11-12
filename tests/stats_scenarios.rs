//! The acceptance scenarios of the `peppec-test stats` command line
//! (specification of #78). Each test is named after its scenario and runs the
//! real binary as a process, with the scenario's arguments and standard input.
//! Standard output, standard error and exit code are checked as the scenario
//! states them; where it says "contains" or "does not mention", only that text
//! is checked.

use std::io::Write;
use std::process::{Command, Output, Stdio};

/// The full output for the numbers 3 1 2.
const FULL_OUTPUT_3_1_2: &str = "count: 3\nmin: 1\nmax: 3\nrange: 2\naverage: 2\nmedian: 2\n";

/// Runs `peppec-test stats` with the arguments. The standard input is `stdin`,
/// which is empty when the scenario gives none, so no test can hang.
fn run(args: &[&str], stdin: &str) -> Output {
    let mut all = vec!["stats"];
    all.extend_from_slice(args);
    run_program(&all, stdin)
}

/// Runs `peppec-test` with exactly the arguments, feeding `stdin` to it.
fn run_program(args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    // The program may exit without reading its input, so a write error is
    // ignored. The inputs are small, so the write cannot block the test.
    let _ = input.write_all(stdin.as_bytes());
    drop(input);
    child.wait_with_output().unwrap()
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8(out.stderr.clone()).unwrap()
}

/// Whether the text has a line that is exactly `line`.
fn has_line(text: &str, line: &str) -> bool {
    text.lines().any(|l| l == line)
}

/// Asserts a success: the exact standard output, an empty standard error and
/// exit code 0.
fn assert_success(args: &[&str], stdin: &str, stdout: &str) {
    let out = run(args, stdin);
    assert_eq!(stdout_of(&out), stdout, "standard output");
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}

/// Asserts a failure: an empty standard output, the exact standard error and
/// exit code 2.
fn assert_failure(args: &[&str], stdin: &str, stderr: &str) {
    let out = run(args, stdin);
    assert_eq!(stdout_of(&out), "", "standard output");
    assert_eq!(stderr_of(&out), stderr, "standard error");
    assert_eq!(out.status.code(), Some(2), "exit code");
}

#[test]
fn all_six_statistics() {
    assert_success(&["3", "1", "2"], "", FULL_OUTPUT_3_1_2);
}

#[test]
fn average_is_truncated() {
    let out = run(&["1", "2"], "");
    let stdout = stdout_of(&out);
    assert!(
        has_line(&stdout, "average: 1"),
        "standard output: {stdout:?}"
    );
    assert!(
        has_line(&stdout, "median: 1.5"),
        "standard output: {stdout:?}"
    );
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}

#[test]
fn negative_average_is_truncated_toward_zero() {
    let out = run(&["-1", "-2"], "");
    let stdout = stdout_of(&out);
    assert!(
        has_line(&stdout, "average: -1"),
        "standard output: {stdout:?}"
    );
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}

#[test]
fn single_number() {
    assert_success(
        &["7"],
        "",
        "count: 1\nmin: 7\nmax: 7\nrange: 0\naverage: 7\nmedian: 7\n",
    );
}

#[test]
fn extremes_of_the_32_bit_range() {
    let out = run(&["-2147483648", "2147483647"], "");
    let stdout = stdout_of(&out);
    assert!(
        has_line(&stdout, "range: 4294967295"),
        "standard output: {stdout:?}"
    );
    assert!(
        has_line(&stdout, "average: 0"),
        "standard output: {stdout:?}"
    );
    assert!(
        has_line(&stdout, "median: -0.5"),
        "standard output: {stdout:?}"
    );
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}

#[test]
fn invalid_number_among_the_arguments() {
    assert_failure(
        &["1", "abc", "3"],
        "",
        "error: 'abc' is not a valid number\n",
    );
}

#[test]
fn number_out_of_range_among_the_arguments() {
    assert_failure(
        &["1", "2147483648"],
        "",
        "error: '2147483648' is out of range for a 32-bit integer\n",
    );
}

#[test]
fn one_number_per_line_on_standard_input() {
    assert_success(&[], "3\n1\n2\n", FULL_OUTPUT_3_1_2);
}

#[test]
fn spaces_and_empty_lines_on_standard_input() {
    assert_success(&[], "3 1\n\n  2\n", FULL_OUTPUT_3_1_2);
}

#[test]
fn invalid_number_on_standard_input() {
    assert_failure(
        &[],
        "1\n2\nabc\n",
        "error: line 3: 'abc' is not a valid number\n",
    );
}

#[test]
fn only_the_first_invalid_value_of_a_line_is_reported_and_empty_lines_are_counted() {
    let out = run(&[], "1\n\n2 x y\n");
    assert_eq!(stdout_of(&out), "", "standard output");
    let stderr = stderr_of(&out);
    assert_eq!(
        stderr, "error: line 3: 'x' is not a valid number\n",
        "standard error"
    );
    assert!(
        !stderr.contains("'y'"),
        "standard error mentions y: {stderr:?}"
    );
    assert_eq!(out.status.code(), Some(2), "exit code");
}

#[test]
fn number_out_of_range_on_standard_input() {
    assert_failure(
        &[],
        "1\n2147483648\n",
        "error: line 2: '2147483648' is out of range for a 32-bit integer\n",
    );
}

#[test]
fn empty_standard_input() {
    assert_failure(&[], "", "error: stats needs at least one number\n");
}

#[test]
fn arguments_win_over_standard_input() {
    let out = run(&["1", "2"], "9\n");
    let stdout = stdout_of(&out);
    assert!(has_line(&stdout, "count: 2"), "standard output: {stdout:?}");
    assert!(has_line(&stdout, "max: 2"), "standard output: {stdout:?}");
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}

#[test]
fn chosen_statistics_come_out_in_canonical_order() {
    assert_success(
        &["--only", "median,count", "3", "1", "2"],
        "",
        "count: 3\nmedian: 2\n",
    );
}

#[test]
fn option_after_the_numbers() {
    assert_success(&["3", "1", "2", "--only", "max"], "", "max: 3\n");
}

#[test]
fn unknown_statistic() {
    assert_failure(
        &["--only", "avg", "1", "2"],
        "",
        "error: unknown statistic 'avg'; available statistics: count, min, max, range, average, median\n",
    );
}

#[test]
fn only_without_a_list() {
    assert_failure(
        &["1", "2", "--only"],
        "",
        "error: --only needs a list of statistics\n",
    );
}

#[test]
fn only_twice() {
    assert_failure(
        &["--only", "min", "--only", "max", "1"],
        "",
        "error: --only given more than once\n",
    );
}

#[test]
fn unknown_option() {
    assert_failure(&["--foo", "1"], "", "error: unknown option '--foo'\n");
}

#[test]
fn json_output() {
    assert_success(
        &["--json", "3", "1", "2"],
        "",
        "{\"count\":3,\"min\":1,\"max\":3,\"range\":2,\"average\":2,\"median\":2}\n",
    );
}

#[test]
fn json_keeps_every_key() {
    assert_success(
        &["--json", "--only", "min,median", "4", "1", "3", "2"],
        "",
        "{\"count\":null,\"min\":1,\"max\":null,\"range\":null,\"average\":null,\"median\":2.5}\n",
    );
}

#[test]
fn errors_stay_plain_text_with_json() {
    assert_failure(
        &["--json"],
        "1\nabc\n",
        "error: line 2: 'abc' is not a valid number\n",
    );
}

#[test]
fn help_lists_the_new_command() {
    let out = run_program(&["--help"], "");
    let stdout = stdout_of(&out);
    assert!(
        stdout.contains("usage: peppec-test <command> [numbers...]"),
        "standard output: {stdout:?}"
    );
    assert!(stdout.contains("stats"), "standard output: {stdout:?}");
    assert!(stdout.contains("--only"), "standard output: {stdout:?}");
    assert!(stdout.contains("--json"), "standard output: {stdout:?}");
    assert_eq!(stderr_of(&out), "", "standard error");
    assert_eq!(out.status.code(), Some(0), "exit code");
}
