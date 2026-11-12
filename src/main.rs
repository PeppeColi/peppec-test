use std::env;
use std::process::ExitCode;

/// Parses one command line argument as a 32-bit signed integer.
///
/// A valid number is an optional `+` or `-` followed by one or more decimal
/// digits, and it must fit in an `i32`.
fn parse_number(text: &str) -> Result<i32, String> {
    let digits = text.strip_prefix(['+', '-']).unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("error: '{text}' is not a valid number"));
    }
    text.parse::<i32>()
        .map_err(|_| format!("error: '{text}' is out of range for a 32-bit integer"))
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    if args.next().as_deref() == Some("median") {
        // `collect` stops at the first invalid value, so only that one is reported.
        let values = match args
            .map(|a| parse_number(&a))
            .collect::<Result<Vec<i32>, _>>()
        {
            Ok(values) => values,
            Err(message) => {
                eprintln!("{message}");
                return ExitCode::from(2);
            }
        };
        match peppec_test::median(&values) {
            Some(result) => println!("{result}"),
            None => {
                eprintln!("error: median needs at least one number");
                return ExitCode::from(2);
            }
        }
    }
    ExitCode::SUCCESS
}
