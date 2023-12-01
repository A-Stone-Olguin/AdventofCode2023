use std::vec;

use crate::helper_functions::io::*;
use regex::Regex;

// Finds the words and matches them with their respective digits
fn word_to_digit(word : &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        // The below takes each "combined" word and puts them into two digits
        "oneight" => "18",
        "twone" => "21",
        "threeight" => "38",
        "sevenine" => "79",
        "nineight" => "98",
        "eightwo" => "82",
        "eighthree" => "83",
        other => other
    }
}

fn make_two_digit_from_first_last(line_string : String) -> String {
    // Match all possible words now, include the combined ones
    let re = match Regex::new(r"(oneight)|(twone)|(threeight)|(sevenine)|(nineight)|(eightwo)|(eighthree)|(\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    // Store the results
    let mut results = vec![];
    for (_, [digit]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        // Get the resultant digit(s)
        let d : &str = word_to_digit(digit);

        // Split the result (If the second is empty, then it had length 1)
        let (first_digit, second_digit) = d.split_at(1);

        // Only had length ==1 means only push the single digit. Otherwise, push both in order.
        if second_digit.is_empty() {
            results.push(first_digit);
        }
        else {
            results.push(first_digit);
            results.push(second_digit);
        }
    }
    // Grab the first and last digits in the vector
    let first : &str = results.first().unwrap();
    let last : &str = results.last().unwrap();

    // Concatenate the two into a string
    (first.to_owned()+last).to_string()      
}



pub fn main() {
    let mut sum : u32 = 0;

    // filenames for input
    let filename = "src/day_01/day_01_input.txt";
    // let filename = "src/day_01/test_02.txt";
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let digits = make_two_digit_from_first_last(ip);
                // Convert the digits into integers
                sum += digits.parse::<u32>().unwrap();
            }
        }
    }
    // Print out the final sum
    println!("The final sum is: {}", sum);
}