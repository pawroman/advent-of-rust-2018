use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader};
use std::ops::AddAssign;
use std::path::PathBuf;

#[macro_use] extern crate failure;
use failure::Error;


#[derive(Debug, Fail)]
#[fail(display = "Invalid number of arguments: {}", num_args)]
struct InvalidArguments {
    num_args: usize,
}


// maximum number of cycles to allow when looking for repeated sums
const MAX_CYCLES: usize = 1000;


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input = get_input(&args);

    if input.is_err() {
        eprintln!("Error: {}. Aborting.", input.unwrap_err());
        std::process::exit(1);
    }

    let frequencies = input.unwrap();

    // part 1

    let freq_sum: i64 = frequencies.iter()
        .sum();

    println!("Sum of frequencies: {}", freq_sum);

    // part 2

    match find_first_cycled_sum_repeat(&frequencies, MAX_CYCLES) {
        Some(repeat) => println!("First repeating frequency: {}", repeat),
        None => {
            println!("No repeats after {} cycles!", MAX_CYCLES);
            std::process::exit(2);
        },
    }

    Ok(())
}


fn get_input(args: &[String]) -> Result<Vec<i64>, Error> {
    let line_inputs;

    match args.len() - 1 {
        0 => {
            eprintln!("Reading input from stdin.");
            line_inputs = get_stdin_input()?;
        },
        arg_idx @ 1 => {
            eprintln!("Reading input from file: `{}'.", args[arg_idx]);
            line_inputs = get_file_input(PathBuf::from(&args[arg_idx]))?;
        },
        num_args => {
            return Err(InvalidArguments { num_args }.into())
        }
    };

    parse_lines(&line_inputs)
}


fn get_stdin_input() -> Result<Vec<String>, Error> {
    let stdin = io::stdin();

    // lock provides thread-safe buffered I/O
    let stdin_lock = stdin.lock();

    read_lines(stdin_lock)
}


fn get_file_input(file_path: PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
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


fn parse_lines(lines: &[String]) -> Result<Vec<i64>, Error> {
    let mut parsed = vec![];

    for line in lines {
        let number = line.parse()?;
        parsed.push(number);
    }

    Ok(parsed)
}


fn find_first_cycled_sum_repeat<T>(values: &[T], max_cycles: usize) -> Option<T>
        where T: AddAssign + Copy + Default + Eq + Hash {
    if values.is_empty() {
        return None;
    }

    let max_iterations = (max_cycles + 1) * values.len();

    let running_cycled_sum = values.iter()
        .cycle()
        .enumerate()
        // starting with default value for T, produce a running sum
        .scan(T::default(), |sum, (iter_no, val)| {
            // max number of cycles reached, stop iteration
            if iter_no >= max_iterations {
                return None;
            }

            *sum += *val;
            Some(*sum)
        });

    let mut seen = HashSet::new();
    seen.insert(T::default());

    running_cycled_sum
        .skip_while(|sum| {
            // insert will return true on duplicate
            seen.insert(*sum)
        })
        .next()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    extern crate tempfile;

    #[test]
    fn test_get_args_file() {
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(b"1\n+2\n-33").unwrap();

        let tmp_file_path = tmp_file.path().to_string_lossy();

        let args: Vec<String> = vec!["prog".into(), tmp_file_path.into()];

        assert_eq!(get_input(&args).unwrap(), vec![1, 2, -33]);
    }

    #[test]
    fn test_parse_lines_all_ok() {
        let input: Vec<String> = vec!["1".into(), "+16".into(), "-42".into()];

        assert_eq!(parse_lines(&input).unwrap(), vec![1, 16, -42]);
    }

    #[test]
    fn test_parse_lines_all_malformed() {
        let input: Vec<String> = vec!["123very".into(), "bad".into(), "inputs666".into()];
        let parsed = parse_lines(&input);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();

        assert!(
            format!("{}", err).starts_with("invalid digit found in string")
        );
    }

    #[test]
    fn test_parse_lines_some_malformed() {
        let input: Vec<String> = vec!["123".into(), "-3-a".into(), "+2".into()];
        let parsed = parse_lines(&input);

        assert!(parsed.is_err());
        let err = parsed.unwrap_err();

        assert!(
            format!("{}", err).starts_with("invalid digit found in string")
        );
    }

    #[test]
    fn test_find_first_cycled_sum_repeat() {
        assert_eq!(
            find_first_cycled_sum_repeat(&[] as &[i32], 0),
            None
        );

        assert_eq!(
            // doesn't need to cycle, first cycling sum is found for last
            find_first_cycled_sum_repeat(&[1, -1], 0),
            Some(0)
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[1, -1], 1),
            Some(0)
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[3, 3, 4, -2, -4], 0),
            None
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[3, 3, 4, -2, -4], 1),
            Some(10)
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[7, 7, -2, -7, -4], 2),
            Some(14)
        );
    }
}
