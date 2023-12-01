use std::vec;

use crate::helper_functions::io::*;
use regex::Regex;

// NOTE: The only difference is changing the regex

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
        other => other
    }
}

fn make_two_digit_from_first_last(line_string : String) -> String {
    // Grab the first letter, going left to right
    let re_lr = match Regex::new(r"(\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut results_lr = vec![];
    for (_, [digit]) in re_lr.captures_iter(&line_string).map(|c| c.extract()) {
        results_lr.push(word_to_digit(digit));
    }
    let first : &str = results_lr.first().unwrap();

    // Makes it so that:
    // 4eightfivefivetwo1oneightvhr
    // matches 48, not 41
    let re_rl = match Regex::new(r"on(eight)|tw(one)|thre(eight)|seve(nine)|nin(eight)|eigh(two)|eigh(three)|(\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut results_rl = vec![];
    for (_, [digit]) in re_rl.captures_iter(&line_string).map(|c| c.extract()) {
        results_rl.push(word_to_digit(digit));
    }
    let last : &str = results_rl.last().unwrap();

    // Concatenate the two into a string
    (first.to_owned()+last).to_string()    
}



pub fn day_01_part_2() {
    let mut sum : u32 = 0;

    // filenames for input
    let filename = "src/day_01/day_01_part_1_input.txt";
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