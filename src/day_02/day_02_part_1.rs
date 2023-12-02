use crate::helper_functions::io::*;
use regex::Regex;

// Compare the colors to the potential comparison values
fn compare_colors(value : &str) -> bool {
    // Comparison values for each color
    let comp_red = 12;
    let comp_green = 13;
    let comp_blue = 14;

    // Split out the matched color and value pair
    let (num, color) = match value.split_once(' ') {
        Some((v, c)) => (v.parse::<i32>().unwrap(),c),
        None => (0, ""),
    };

    // Returns if the color fits the comparison value
    match color {
        "green" => num <= comp_green,
        "red" => num <= comp_red,
        "blue" => num <= comp_blue,
        _ => false
    }

}

// Returns the game number if it is valid
fn get_valid_game_number(line_string: String) -> i32 {
    // Get the Game number
    let re = match Regex::new(r"Game (\d+):") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut game_num : i32 = 0; 
    for (_, [digit]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        game_num = digit.parse::<i32>().unwrap();
    }

    // Get each color and their value
    let re_color = match Regex::new(r"(\d+ blue)|(\d+ green)|(\d+ red)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    for (_, [game]) in re_color.captures_iter(&line_string).map(|c| c.extract()) {
        // If the colors fail a comparison, return the additive identity
        if !compare_colors(game) {
            return 0;
        }
    }
    game_num


}


// Main function for day 2
pub fn main() {
    // filenames for input
    let filename = "src/day_02/day_02_input.txt";
    // let filename = "src/day_02/test_01.txt";

    let mut sum = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Add the game id for each valid game
                sum += get_valid_game_number(ip);
            }
        }
    }
    println!("The final value of possible games is: {}", sum)
}