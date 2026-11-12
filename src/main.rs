use std::env;
use std::process::ExitCode;

/// A command of the command line.
///
/// `run` receives the arguments after the command name and returns the text
/// to print on success, or the error message to print on failure.
struct Command {
    name: &'static str,
    summary: &'static str,
    run: fn(&[String]) -> Result<String, String>,
}

/// Every command the program knows. Adding a command means adding an entry here.
const COMMANDS: &[Command] = &[Command {
    name: "median",
    summary: "print the median of the numbers",
    run: run_median,
}];

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

fn run_median(args: &[String]) -> Result<String, String> {
    // `collect` stops at the first invalid value, so only that one is reported.
    let values = args
        .iter()
        .map(|a| parse_number(a))
        .collect::<Result<Vec<i32>, _>>()?;
    peppec_test::median(&values)
        .map(|result| result.to_string())
        .ok_or_else(|| "error: median needs at least one number".to_owned())
}

fn command_names() -> String {
    COMMANDS
        .iter()
        .map(|c| c.name)
        .collect::<Vec<_>>()
        .join(", ")
}

fn usage() -> String {
    let commands: String = COMMANDS
        .iter()
        .map(|c| format!("  {:<8}{}\n", c.name, c.summary))
        .collect();
    format!(
        "usage: peppec-test <command> [numbers...]\n\
         \n\
         commands:\n\
         {commands}\n\
         options:\n\
         \x20 --help    show this help\n"
    )
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.split_first() {
        None => {
            eprint!("{}", usage());
            ExitCode::from(2)
        }
        Some((first, _)) if first.as_str() == "--help" => {
            print!("{}", usage());
            ExitCode::SUCCESS
        }
        Some((name, rest)) => {
            let name = name.as_str();
            let Some(command) = COMMANDS.iter().find(|c| c.name == name) else {
                eprintln!(
                    "error: unknown command '{name}'; available commands: {}",
                    command_names()
                );
                return ExitCode::from(2);
            };
            match (command.run)(rest) {
                Ok(output) => {
                    println!("{output}");
                    ExitCode::SUCCESS
                }
                Err(message) => {
                    eprintln!("{message}");
                    ExitCode::from(2)
                }
            }
        }
    }
}
