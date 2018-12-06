extern crate assert_cmd;
extern crate tempfile;


#[cfg(test)]
mod cli {
    use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;


    #[test]
    fn test_run_input_file() {
        // the actual example file from Advent of Code

        let mut cmd = Command::main_binary().unwrap();

        // read in the example input, write to tmp file and run the command
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.write_all(include_str!("../input/input").as_bytes()).unwrap();

        cmd.arg(&tmp_file.path());

        cmd
            .assert()
            .success()
            .stdout("Checksum: 6225\n\
                     Common part: revtaubfniyhsgxdoajwkqilp\n");
    }
}
