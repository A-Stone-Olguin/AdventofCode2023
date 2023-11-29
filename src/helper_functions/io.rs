use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(filename: &str) {

    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}\n", display, s),
    }

    // `file` goes out of scope, and the file gets closed
}

pub fn read_lines<P>(filename : P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = match File::open(filename) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };
        Ok(io::BufReader::new(file).lines())
    }
