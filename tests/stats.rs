use std::process::Command;

fn run(args: &[&str]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_peppec-test"))
        .arg("stats")
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
        out.status.code(),
    )
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
