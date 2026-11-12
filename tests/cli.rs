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
