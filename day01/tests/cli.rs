extern crate assert_cmd;
extern crate tempfile;


#[cfg(test)]
mod cli {
    use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn test_run_stdin() {
        let mut cmd = Command::main_binary()
            .unwrap();

        // this weird syntax is circumventing a bug in assert_cmd

        let mut stdin_cmd = cmd
            .with_stdin();

        let mut assert_cmd = stdin_cmd
            .buffer("+3\n+3\n+4\n-2\n-4");

        let assert = assert_cmd.assert();

        assert
            .success()
            .stderr("Reading input from stdin.\n")
            // \ breaks the string without spaces and indents
            .stdout("Sum of frequencies: 4\n\
                     First repeating frequency: 10\n");
    }

    #[test]
    fn test_run_input_file() {
        // the actual example file from Advent of Code

        let mut cmd = Command::main_binary().unwrap();

        // read in the example input, write to tmp file and run the command
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(include_str!("../input/input").as_bytes()).unwrap();

        let tmp_file_path = tmp_file.path().to_string_lossy();

        cmd.arg(tmp_file_path.into_owned());

        let assert = cmd.assert();

        assert
            .success()
            .stdout("Sum of frequencies: 592\n\
                     First repeating frequency: 241\n");
    }

    #[test]
    fn test_invalid_num_of_args() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg("blah")
            .arg("blah");

        let assert = cmd.assert();

        assert
            .failure()
            .stderr("Error: Invalid number of arguments: 2. Aborting.\n");
    }
}
