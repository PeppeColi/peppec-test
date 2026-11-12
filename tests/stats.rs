use std::io::Write;
use std::process::{Command, Stdio};

const FULL_OUTPUT: &str = "count: 3\nmin: 1\nmax: 3\nrange: 2\naverage: 2\nmedian: 2\n";

/// Runs `peppec-test stats` with the arguments, feeding `input` to its
/// standard input, and returns standard output, standard error and exit code.
fn run_with_input(args: &[&str], input: &[u8]) -> (String, String, Option<i32>) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .arg("stats")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let input = input.to_vec();
    // Write from another thread so that a full pipe cannot block the test. The
    // program may exit without reading its input, so a write error is ignored.
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&input);
    });
    let out = child.wait_with_output().unwrap();
    writer.join().unwrap();
    (
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
        out.status.code(),
    )
}

/// Runs `peppec-test stats` with the arguments and an empty standard input.
fn run(args: &[&str]) -> (String, String, Option<i32>) {
    run_with_input(args, b"")
}

fn ok(args: &[&str]) -> String {
    let (out, err, code) = run(args);
    assert!(err.is_empty());
    assert_eq!(code, Some(0));
    out
}

fn fails(args: &[&str], message: &str) {
    let (out, err, code) = run(args);
    assert!(out.is_empty());
    assert_eq!(err, format!("{message}\n"));
    assert_eq!(code, Some(2));
}

fn ok_input(input: &[u8]) -> String {
    let (out, err, code) = run_with_input(&[], input);
    assert!(err.is_empty(), "standard error: {err}");
    assert_eq!(code, Some(0));
    out
}

fn fails_input(args: &[&str], input: &[u8], message: &str) {
    let (out, err, code) = run_with_input(args, input);
    assert!(out.is_empty());
    assert_eq!(err, format!("{message}\n"));
    assert_eq!(code, Some(2));
}

#[test]
fn prints_six_statistics() {
    assert_eq!(
        ok(&["3", "1", "2"]),
        "count: 3\nmin: 1\nmax: 3\nrange: 2\naverage: 2\nmedian: 2\n"
    );
}

#[test]
fn average_truncates_and_median_is_fractional() {
    let out = ok(&["1", "2"]);
    assert!(out.contains("average: 1\n"));
    assert!(out.contains("median: 1.5\n"));
}

#[test]
fn average_truncates_toward_zero() {
    assert!(ok(&["-1", "-2"]).contains("average: -1\n"));
}

#[test]
fn single_number() {
    assert_eq!(
        ok(&["7"]),
        "count: 1\nmin: 7\nmax: 7\nrange: 0\naverage: 7\nmedian: 7\n"
    );
}

#[test]
fn extremes_of_32_bit_range() {
    let out = ok(&["-2147483648", "2147483647"]);
    assert!(out.contains("range: 4294967295\n"));
    assert!(out.contains("average: 0\n"));
    assert!(out.contains("median: -0.5\n"));
}

#[test]
fn invalid_number() {
    fails(&["1", "abc", "3"], "error: 'abc' is not a valid number");
}

#[test]
fn out_of_range_number() {
    fails(
        &["1", "2147483648"],
        "error: '2147483648' is out of range for a 32-bit integer",
    );
}

#[test]
fn no_numbers() {
    fails(&[], "error: stats needs at least one number");
}

#[test]
fn stdin_one_number_per_line() {
    assert_eq!(ok_input(b"3\n1\n2\n"), FULL_OUTPUT);
}

#[test]
fn stdin_spaces_and_empty_lines() {
    assert_eq!(ok_input(b"3 1\n\n  2\n"), FULL_OUTPUT);
}

#[test]
fn stdin_tab_and_no_final_newline() {
    assert_eq!(ok_input(b"3\t1 2"), FULL_OUTPUT);
}

#[test]
fn stdin_crlf_line_endings() {
    assert_eq!(ok_input(b"3\r\n1\r\n2\r\n"), FULL_OUTPUT);
}

#[test]
fn stdin_invalid_number() {
    fails_input(
        &[],
        b"1\n2\nabc\n",
        "error: line 3: 'abc' is not a valid number",
    );
}

#[test]
fn stdin_first_invalid_value_and_empty_lines_counted() {
    let (out, err, code) = run_with_input(&[], b"1\n\n2 x y\n");
    assert!(out.is_empty());
    assert_eq!(err, "error: line 3: 'x' is not a valid number\n");
    assert!(!err.contains('y'));
    assert_eq!(code, Some(2));
}

#[test]
fn stdin_out_of_range_number() {
    fails_input(
        &[],
        b"1\n2147483648\n",
        "error: line 2: '2147483648' is out of range for a 32-bit integer",
    );
}

#[test]
fn stdin_empty() {
    fails_input(&[], b"", "error: stats needs at least one number");
}

#[test]
fn stdin_only_whitespace() {
    fails_input(&[], b"\n  \n", "error: stats needs at least one number");
}

#[test]
fn stdin_not_utf8() {
    fails_input(&[], b"\xFF", "error: input is not valid UTF-8");
}

#[test]
fn arguments_win_over_stdin() {
    let (out, err, code) = run_with_input(&["1", "2"], b"9\n");
    assert!(err.is_empty());
    assert!(out.contains("count: 2\n"));
    assert!(out.contains("max: 2\n"));
    assert_eq!(code, Some(0));
}

#[test]
fn invalid_argument_has_no_line_even_with_stdin() {
    fails_input(&["1", "abc"], b"9\n", "error: 'abc' is not a valid number");
}

#[test]
fn only_prints_chosen_statistics_in_canonical_order() {
    assert_eq!(
        ok(&["--only", "median,count", "3", "1", "2"]),
        "count: 3\nmedian: 2\n"
    );
    assert_eq!(ok(&["3", "1", "2", "--only", "max"]), "max: 3\n");
    assert_eq!(ok(&["3", "--only", "range", "1", "2"]), "range: 2\n");
    assert_eq!(ok(&["--only", "min,min", "3", "1", "2"]), "min: 1\n");
    assert_eq!(ok(&["--only", "min", "-5", "-1"]), "min: -5\n");
}

#[test]
fn only_reads_standard_input_when_no_number_is_given() {
    let (out, err, code) = run_with_input(&["--only", "max"], b"3\n1\n2\n");
    assert_eq!(out, "max: 3\n");
    assert!(err.is_empty());
    assert_eq!(code, Some(0));
}

#[test]
fn option_errors() {
    let list = "available statistics: count, min, max, range, average, median";
    fails(
        &["--only", "avg", "1", "2"],
        &format!("error: unknown statistic 'avg'; {list}"),
    );
    fails(
        &["--only", "1", "2"],
        &format!("error: unknown statistic '1'; {list}"),
    );
    fails(
        &["--only", "min,,max", "1"],
        &format!("error: unknown statistic ''; {list}"),
    );
    fails(
        &["1", "2", "--only"],
        "error: --only needs a list of statistics",
    );
    fails(
        &["--only", "", "1", "2"],
        "error: --only needs a list of statistics",
    );
    fails(
        &["--only", "min", "--only", "max", "1"],
        "error: --only given more than once",
    );
    fails(&["--foo", "1"], "error: unknown option '--foo'");
    fails(&["--only=min", "1"], "error: unknown option '--only=min'");
    fails(&["--help"], "error: unknown option '--help'");
    fails(&["--foo", "abc"], "error: unknown option '--foo'");
}

const JSON_ALL: &str = r#"{"count":3,"min":1,"max":3,"range":2,"average":2,"median":2}"#;

#[test]
fn json_prints_one_object_with_six_keys() {
    assert_eq!(ok(&["--json", "3", "1", "2"]), format!("{JSON_ALL}\n"));
}

#[test]
fn json_option_may_come_after_numbers() {
    assert_eq!(ok(&["3", "1", "2", "--json"]), format!("{JSON_ALL}\n"));
}

#[test]
fn json_given_twice_changes_nothing() {
    assert_eq!(
        ok(&["--json", "--json", "3", "1", "2"]),
        format!("{JSON_ALL}\n")
    );
}

#[test]
fn json_with_only_writes_null_for_other_statistics() {
    assert_eq!(
        ok(&["--json", "--only", "min,median", "4", "1", "3", "2"]),
        "{\"count\":null,\"min\":1,\"max\":null,\"range\":null,\"average\":null,\"median\":2.5}\n"
    );
}

#[test]
fn json_extremes_of_32_bit_range() {
    assert_eq!(
        ok(&["--json", "-2147483648", "2147483647"]),
        "{\"count\":2,\"min\":-2147483648,\"max\":2147483647,\"range\":4294967295,\"average\":0,\"median\":-0.5}\n"
    );
}

#[test]
fn json_reads_standard_input_when_no_number_is_given() {
    let (out, err, code) = run_with_input(&["--json"], b"3\n1\n2\n");
    assert_eq!(out, format!("{JSON_ALL}\n"));
    assert!(err.is_empty());
    assert_eq!(code, Some(0));
}

#[test]
fn json_stdin_invalid_number_is_plain_error() {
    fails_input(
        &["--json"],
        b"1\nabc\n",
        "error: line 2: 'abc' is not a valid number",
    );
}

#[test]
fn json_unknown_statistic_is_plain_error() {
    fails(
        &["--json", "--only", "avg", "1"],
        "error: unknown statistic 'avg'; available statistics: count, min, max, range, average, median",
    );
}

const USAGE_LINE: &str = "usage: peppec-test <command> [numbers...]";

/// Runs `peppec-test` itself, without the `stats` command name, and returns
/// standard output, standard error and exit code.
fn run_program(args: &[&str]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
        out.status.code(),
    )
}

#[test]
fn help_lists_commands_and_options() {
    let (out, err, code) = run_program(&["--help"]);
    assert!(
        out.starts_with(&format!("{USAGE_LINE}\n")),
        "standard output: {out:?}"
    );
    for expected in [
        "median", "stats", "--only", "--json", "count", "min", "max", "range", "average",
    ] {
        assert!(out.contains(expected), "standard output: {out:?}");
    }
    assert!(err.is_empty(), "standard error: {err:?}");
    assert_eq!(code, Some(0));
}

#[test]
fn help_mentions_standard_input() {
    let (out, _, _) = run_program(&["--help"]);
    assert!(out.contains("standard input"), "standard output: {out:?}");
}

#[test]
fn no_arguments_prints_stats_usage_to_error_stream() {
    let (out, err, code) = run_program(&[]);
    assert!(out.is_empty(), "standard output: {out:?}");
    assert!(err.starts_with(USAGE_LINE), "standard error: {err:?}");
    for expected in ["stats", "--only", "--json"] {
        assert!(err.contains(expected), "standard error: {err:?}");
    }
    assert_eq!(code, Some(2));
}
