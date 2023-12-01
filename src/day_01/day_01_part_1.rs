use std::vec;

use crate::helper_functions::io::*;
use regex::Regex;

// Makes two digits from the first and last digit in a line
fn make_two_digit_from_first_last(line_string : String) -> String {
    // Regex to grab digits
    let re = match Regex::new(r"(\d)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    // Store the results in a vector to grab the first and last digits
    let mut results = vec![];
    for (_, [digit]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        results.push(digit);
    }
    let first : &str = results.first().unwrap();
    let last : &str = results.last().unwrap();

    // Return the string-concatenated digits
    (first.to_owned()+last).to_string()    
}


// Main function for day 1
pub fn main() {
    let mut sum : u32 = 0;
    if let Ok(lines) = read_lines("./src/day_01/day_01_part_1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let digits = make_two_digit_from_first_last(ip);
                // Add up all the two-digit values
                sum += digits.parse::<u32>().unwrap();
            }
        }
    }
    println!("The final sum is: {}", sum);
}