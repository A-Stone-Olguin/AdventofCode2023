use crate::helper_functions::io::*;

// Main function for day 2
pub fn main() {
    // filenames for input
    let filename = "src/day_02/day_02_input.txt";
    // let filename = "src/day_02/test_02.txt";

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}