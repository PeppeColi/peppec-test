use std::env;
use std::io::{self, Read};
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
const COMMANDS: &[Command] = &[
    Command {
        name: "median",
        summary: "print the median of the numbers",
        run: run_median,
    },
    Command {
        name: "stats",
        summary: "print count, min, max, range, average and median",
        run: run_stats,
    },
];

/// Parses one command line argument as a 32-bit signed integer.
///
/// A valid number is an optional `+` or `-` followed by one or more decimal
/// digits, and it must fit in an `i32`. The error describes the problem
/// without the `error: ` prefix, so that callers can add where it is.
fn parse_value(text: &str) -> Result<i32, String> {
    let digits = text.strip_prefix(['+', '-']).unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("'{text}' is not a valid number"));
    }
    text.parse::<i32>()
        .map_err(|_| format!("'{text}' is out of range for a 32-bit integer"))
}

/// Parses one command line argument as a number, with the full error message.
fn parse_number(text: &str) -> Result<i32, String> {
    parse_value(text).map_err(|problem| format!("error: {problem}"))
}

/// Parses the numbers of a text, separated by any whitespace.
///
/// Lines are counted from 1, empty lines included, and only the first invalid
/// value in reading order is reported, with its line.
fn parse_text(text: &str) -> Result<Vec<i32>, String> {
    let mut values = Vec::new();
    for (index, line) in text.split('\n').enumerate() {
        for word in line.split_whitespace() {
            let value = parse_value(word)
                .map_err(|problem| format!("error: line {}: {problem}", index + 1))?;
            values.push(value);
        }
    }
    Ok(values)
}

/// Reads standard input until its end and parses the numbers in it.
fn read_stdin_numbers() -> Result<Vec<i32>, String> {
    let mut bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .map_err(|e| format!("error: cannot read standard input: {e}"))?;
    let text =
        String::from_utf8(bytes).map_err(|_| "error: input is not valid UTF-8".to_owned())?;
    parse_text(&text)
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

/// The statistics of `stats`, in canonical order.
const STATISTICS: [&str; 6] = ["count", "min", "max", "range", "average", "median"];

/// Parses the list given to `--only` into one flag per statistic, in
/// canonical order. The first unknown name, left to right, is reported.
fn parse_only(list: &str) -> Result<[bool; 6], String> {
    if list.is_empty() {
        return Err("error: --only needs a list of statistics".to_owned());
    }
    let mut chosen = [false; 6];
    for name in list.split(',') {
        let Some(index) = STATISTICS.iter().position(|s| *s == name) else {
            return Err(format!(
                "error: unknown statistic '{name}'; available statistics: {}",
                STATISTICS.join(", ")
            ));
        };
        chosen[index] = true;
    }
    Ok(chosen)
}

/// Writes the statistics as one JSON object with all six keys, in canonical
/// order. A statistic that is not chosen is written as `null`. The object is
/// written by hand: the values are plain numbers, so nothing needs escaping.
fn json_object(results: &[String; 6], chosen: [bool; 6]) -> String {
    let fields: Vec<String> = STATISTICS
        .iter()
        .zip(results)
        .zip(chosen)
        .map(|((name, value), on)| {
            let value = if on { value.as_str() } else { "null" };
            format!("\"{name}\":{value}")
        })
        .collect();
    format!("{{{}}}", fields.join(","))
}

fn run_stats(args: &[String]) -> Result<String, String> {
    // Options are recognised first, so their problems come before those of the
    // numbers and before standard input is read.
    let mut chosen: Option<[bool; 6]> = None;
    let mut json = false;
    let mut numbers: Vec<&String> = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--json" {
            json = true;
        } else if arg == "--only" {
            if chosen.is_some() {
                return Err("error: --only given more than once".to_owned());
            }
            let list = iter
                .next()
                .ok_or_else(|| "error: --only needs a list of statistics".to_owned())?;
            chosen = Some(parse_only(list)?);
        } else if arg.starts_with("--") {
            return Err(format!("error: unknown option '{arg}'"));
        } else {
            numbers.push(arg);
        }
    }
    let chosen = chosen.unwrap_or([true; 6]);
    // Arguments and standard input are never mixed: standard input is read
    // only when no number is given as an argument.
    let values = if numbers.is_empty() {
        read_stdin_numbers()?
    } else {
        numbers
            .iter()
            .map(|a| parse_number(a))
            .collect::<Result<Vec<i32>, _>>()?
    };
    let empty = || "error: stats needs at least one number".to_owned();
    let (min, max) = peppec_test::min_max(&values).ok_or_else(empty)?;
    let range = peppec_test::range(&values).ok_or_else(empty)?;
    let median = peppec_test::median(&values).ok_or_else(empty)?;
    let results = [
        values.len().to_string(),
        min.to_string(),
        max.to_string(),
        range.to_string(),
        peppec_test::average(&values).to_string(),
        median.to_string(),
    ];
    if json {
        return Ok(json_object(&results, chosen));
    }
    let lines: Vec<String> = STATISTICS
        .iter()
        .zip(&results)
        .zip(chosen)
        .filter(|(_, on)| *on)
        .map(|((name, value), _)| format!("{name}: {value}"))
        .collect();
    Ok(lines.join("\n"))
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
         stats reads the numbers from standard input when none are given as arguments.\n\
         \n\
         options:\n\
         \x20 --help         show this help\n\
         \x20 --only <list>  stats: show only the listed statistics, separated by commas:\n\
         \x20                count, min, max, range, average, median\n\
         \x20 --json         stats: print the statistics as one JSON object\n"
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
