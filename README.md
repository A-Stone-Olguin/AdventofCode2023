# AdventofCode2023
Solutions for the 2023 Advent of Code

## How to Run:
* First, make sure rust and cargo are installed:
  * Rust: https://doc.rust-lang.org/book/ch01-01-installation.html
  * After installation, make sure Cargo is installed by running `cargo --version`
  
To run a particular day and part file, simply type 
```
cargo run -- --day <Day_Number> --part <Part_Number>
```

For example, to run Day 2's second part:
```
cargo run -- --day 2 --part 2
```

## Setting up a new day:
To make it easy to set up a new day's template, run the bash script: `make_day.sh`

This will set up the necessary files for a specified day with a given input. 
For example:
```
./make_day.sh 5
```
Will create a directory `src/day_05/` which will include rust files for each part of each day

This will not update `src/main.rs` to include the `crate::day_0X::*;` lines or update as a valid input in the `run_day_part` function.

NOTE: This currently does not check if there is already existing files.
This will be implemented in the future, for now please be careful using this script.

## Future Improvements:
* Check for if files exist in the bash script
* Find a way to automatically update `src/main.rs` to include:
  * `crate::day_0X::*;` at the beginning of the file
  * `run_day_part` function to include additional cases as valid day/part pairs