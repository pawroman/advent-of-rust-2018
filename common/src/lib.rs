use std::io::{self, BufRead, BufReader};
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

mod errors;

pub use crate::errors::Error;

use crate::errors::{Fail, InvalidArguments};


pub fn get_input<T, U>(args: &[U]) -> Result<Vec<T>, Error>
    where T: FromStr,
          <T as FromStr>::Err: Fail,
          U: AsRef<str> + AsRef<Path> + Display
{
    let line_inputs;

    match args.len() - 1 {
        0 => {
            eprintln!("Reading input from stdin.");
            line_inputs = get_stdin_input()?;
        },
        arg_idx @ 1 => {
            eprintln!("Reading input from file: `{}'.", args[arg_idx]);
            line_inputs = get_file_input(&args[arg_idx])?;
        },
        num_args => {
            return Err(InvalidArguments { num_args }.into())
        }
    };

    parse_lines::<T, _>(&line_inputs)
}


fn get_stdin_input() -> Result<Vec<String>, Error> {
    let stdin = io::stdin();

    // lock provides thread-safe buffered I/O
    let stdin_lock = stdin.lock();

    read_lines(stdin_lock)
}


fn get_file_input<T>(file_path: T) -> Result<Vec<String>, Error>
    where T: AsRef<Path>
{
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    read_lines(reader)
}


fn read_lines(reader: impl BufRead) -> Result<Vec<String>, Error> {
    let mut lines = vec![];

    for line in reader.lines() {
        // bail on first error
        lines.push(line?);
    }

    Ok(lines)
}


fn parse_lines<T, U>(lines: &[U]) -> Result<Vec<T>, Error>
    where T: FromStr,
          <T as FromStr>::Err: Fail,
          U: AsRef<str>
{
    let mut parsed: Vec<T> = vec![];

    for line in lines {
        let number = line.as_ref().parse()?;
        parsed.push(number);
    }

    Ok(parsed)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    use tempfile;

    #[test]
    fn test_get_args_file() {
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(b"1\n+2\n-33").unwrap();

        let tmp_file_path = tmp_file.path();

        let args = ["prog", tmp_file_path.to_str().unwrap()];

        assert_eq!(get_input::<i64, _>(&args).unwrap(), vec![1, 2, -33]);
    }

    #[test]
    fn test_parse_lines_all_ok() {
        let input = ["1", "+16", "-42"];

        assert_eq!(parse_lines::<i64, _>(&input).unwrap(), vec![1, 16, -42]);
    }

    #[test]
    fn test_parse_lines_all_malformed() {
        let input = ["123very", "bad", "inputs666"];
        let parsed = parse_lines::<i64, _>(&input);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();

        assert!(
            format!("{}", err).starts_with("invalid digit found in string")
        );
    }

    #[test]
    fn test_parse_lines_some_malformed() {
        let input = ["123", "-3-a", "+2"];
        let parsed = parse_lines::<i64, _>(&input);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();

        assert!(
            format!("{}", err).starts_with("invalid digit found in string")
        );
    }

    #[test]
    fn test_parse_lines_works_with_vector_of_strings() {
        // testing generic code sanity
        let input: Vec<String> = vec!["1".into(), "+16".into(), "-42".into()];

        assert_eq!(parse_lines::<i64, _>(&input).unwrap(), vec![1, 16, -42]);
    }

    #[test]
    fn test_parse_lines_works_with_f32() {
        let input = ["123.5", "-3.5", "+2.0"];

        assert_eq!(parse_lines::<f32, _>(&input).unwrap(),
                   vec![123.5, -3.5, 2.0]);
    }
}
