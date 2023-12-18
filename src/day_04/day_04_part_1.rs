use crate::helper_functions::io::*;
use regex::Regex;
use std::vec;

// Get the lucky numbers and your numbers from the input
fn get_lucky_numbers_and_your_numbers(line_string : String) -> (i32, Vec<i32>, Vec<i32>) {
    let re = match Regex::new(r"Card\s+(\d+):\s+((?:\d+\s+)+)\|\s+((?:\d+\s*)+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut your_nums_vec = vec![];
    let mut lucky_nums_vec = vec![];
    let mut game_num = 0;
    for (_, [game_n, lucky_nums, your_nums]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        // Note the vectors are in reverse order!
        let re_space = match Regex::new(r"\s+") {
            Ok(r) => r,
            Err(msg) => panic!("Error: {}", msg),
        };
        let l_v: Vec<&str> = re_space.split(lucky_nums.trim()).collect();
        let y_v: Vec<&str> = re_space.split(your_nums.trim()).collect();
        lucky_nums_vec = l_v.iter().map(|&c| c.parse::<i32>().unwrap()).collect();
        your_nums_vec = y_v.iter().map(|&c| c.parse::<i32>().unwrap()).collect();
        game_num = game_n.parse::<i32>().unwrap();
    }
    (game_num, lucky_nums_vec, your_nums_vec)
}

// Determine your score
fn calculate_score(line_string : String) ->i32 {
    let (_game_num, lucky_nums, your_nums) = get_lucky_numbers_and_your_numbers(line_string);
    let mut score = 0;
    for your_num in your_nums.iter() {
        for lucky_num in lucky_nums.iter() {
            if lucky_num == your_num {
                score = match score {
                    0 => 1,
                    s => 2 * s,
                }
            }
        }
    }
    score
}

// Main function for day 4
pub fn main() {
  //filenames for input
  let filename = "src/day_04/day_04_input.txt";
//   let filename = "src/day_04/test_01.txt";

  let mut sum = 0;
  if let Ok(lines) = read_lines(filename) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(ip) = line {
            sum += calculate_score(ip);
          }
      }
  }
  println!("The final score is: {}", sum)
}
