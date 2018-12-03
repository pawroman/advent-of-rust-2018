# Advent of code 2018

[![Build Status](https://travis-ci.org/pawroman/advent-of-rust-2018.svg?branch=master)](https://travis-ci.org/pawroman/advent-of-rust-2018)

My Rust solutions to Advent of Code 2018 challenges:
https://adventofcode.com/2018

## Design choices

Although one can devise much more succinct solutions
to some problems (write code in ad-hoc script
style), my motivation was to:

* Be correct and write the code such that it can be
  easily tested.  All the code has tests (test
  coverage might not be 100%, but it's pretty solid).
  
* Apply defensive programming (assume the challenge
  can have malformed input or might never
  converge to a solution).
  
* Learn more advanced Rust, in particular:

    * Testing.

    * "Production grade" error handling (never
      panic, always return structured errors).
      
    * Generic programming.

## Running

To build & run, make sure you have Rust 1.30.1 or
above.

All code has tests, you can run all of them using:

```bash
$ cargo test
```

To run one a challenge, e.g. `day01`, you need
to specify its name and path to input:

```bash
$ cargo run --bin day01 day01/input/input
```

Or to run in release mode:

```bash
$ cargo run --release --bin day01 day01/input/input
```
