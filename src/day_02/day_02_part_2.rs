use crate::helper_functions::io::*;
use regex::Regex;

// Calculates the set's power
fn calculate_set_power(line_string: String) -> i32 {
    // Grab each color and their values
    let re_color = match Regex::new(r"(\d+ blue)|(\d+ green)|(\d+ red)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    // Grab each set of values, will eventually find the maximum
    let mut green_vals = vec![];
    let mut red_vals = vec![];
    let mut blue_vals = vec![];
    for (_, [game]) in re_color.captures_iter(&line_string).map(|c| c.extract()) {
        // Split out each color and its associated value
        let (num, color) = match game.split_once(' ') {
            Some((v, c)) => (v.parse::<i32>().unwrap(),c),
            None => (0, ""),
        };
        // Push the corresponding value to its vector
        match color {
            "green" => green_vals.push(num),
            "red" => red_vals.push(num),
            "blue" => blue_vals.push(num),
            _ => ()
        }
    }
    // Calculate each color's maximum to determine the power
    let max_red = match red_vals.iter().max() {
        Some(max) => max,
        None => &1, // An error, put to multiplicative identity to not affect power calculation
    };
    let max_blue = match blue_vals.iter().max() {
        Some(max) => max,
        None => &1, // An error, put to multiplicative identity to not affect power calculation
    };
    let max_green = match green_vals.iter().max() {
        Some(max) => max,
        None => &1, // An error, put to multiplicative identity to not affect power calculation
    };
    max_red*max_blue*max_green



}

// Main function for day 2
pub fn main() {
    // filenames for input
    let filename = "src/day_02/day_02_input.txt";
    // let filename = "src/day_02/test_02.txt";

    let mut sum = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Sum up each game's power
                sum += calculate_set_power(ip);
            }
        }
    }
    println!("The final value of powers' sum is: {}", sum)
}