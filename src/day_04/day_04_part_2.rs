use crate::helper_functions::io::*;
use regex::Regex;
use core::panic;
use std::vec;

// Get the lucky numbers and your numbers
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

// Calculate the score of the number of matches. Modified from part 1.
fn calculate_score(line_string : String) ->(i32, i32) {
    let (game_num, lucky_nums, your_nums) = get_lucky_numbers_and_your_numbers(line_string);
    let mut num_matches = 0;
    for your_num in your_nums.iter() {
        for lucky_num in lucky_nums.iter() {
            if lucky_num == your_num {
                num_matches += 1;
            }
        }
    }
    (game_num, num_matches)
}

// Sum the number of lucky cards we have
fn sum_num_lucky_cards(vals : Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    // Scalars will be how many duplicate cards we have, initially all zero
    let mut copies :Vec<i32> = vec![];
    for _ in 0..vals.len() {
        copies.push(0);
    }

    for i in 0..vals.len() {
        let (_game, num_matches) = &vals[i];
        // Add up each copy and add copies for future runs:
        for j in 0..*num_matches {
            // Increment the number of cards we have
            copies[i + j as usize + 1]+= 1 + copies[i];
            sum+= 1 + copies[i]
        }
        // Add the original card
        sum += 1;
    }
    sum

}

// Main function for day 4
pub fn main() {
  //filenames for input
  let filename = "src/day_04/day_04_input.txt";
//   let filename = "src/day_04/test_02.txt";

  let mut sum = 0;
  let mut vals :Vec<(i32, i32)> = vec![];
  if let Ok(lines) = read_lines(filename) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(ip) = line {
            vals.push(calculate_score(ip));
          }
      }
  }
  sum += sum_num_lucky_cards(vals);

  println!("The final score is: {}", sum)
}
