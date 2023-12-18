use crate::helper_functions::io::*;

use regex::Regex;

// Function that determines each symbol's location
fn get_symbol_locations_row_i(line_string : String) -> Vec<i32> {
    // Capture the symbols in the row
    let re = match Regex::new(r"([^\d\.])") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut indices = vec![];
    // Push each capture's location to the indices matrix
    for capture in re.captures_iter(&line_string) {
        indices.push(capture.get(0).unwrap().start() as i32);
    }
    indices
}

// Get the (value, start_index, end_index) for each number in the row
fn get_number_locations_row_i(line_string : String) -> Vec<(i32, i32, i32)> {
    // Capture each number
    let re = match Regex::new(r"(\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut num_locations_info = vec![];

    // Grab the locations for each capture
    let mut locs = re.capture_locations();
    let _m = match re.captures_read(&mut locs, &line_string) {
        Some(m) => m,
        None => return num_locations_info,
    };

    // For each capture, grab the value, start index, and end index
    for capture in re.captures_iter(&line_string) {
        let value = capture[1].parse::<i32>().unwrap();
        let start = capture.get(0).unwrap().start() as i32;
        let end = capture.get(0).unwrap().end() as i32 - 1;
        num_locations_info.push((value, start, end));
    }
    num_locations_info

}

// For a given value and its indices, determine if it has a symbol near it
fn determine_valid_numbers(row : usize, start: i32, end :i32, matrix : Vec<Vec<i32>>) -> bool {
    for i in start..end+1 {
        // Determine the previous row, 0 if its the end
        let lower_bound = match row {
            0 => row,
            _ => row-1,
        };
        // Go through previous, current, and next row
        for row_num in lower_bound..row+2 {
            // Grab the corresponding row in the matrix
            let matrix_row = match matrix.get(row_num) {
                Some(mr) => mr.clone(),
                None => Vec::new(),
            };
            // Determine if there is a symbol near the current index i
            for j in matrix_row {
                match i - 1 == j || i + 1 == j || i == j {
                    true => return true,
                    false => (),
                }
            }
        }
    }
    // If we found no valid number, return false
    false
}

// Determine if a number is valid and sum the value if it is
fn sum_valid_numbers(matrix : Vec<Vec<i32>>, values_data : Vec<Vec<(i32, i32, i32)>>) -> i32 {
  let mut sum = 0;
  for row in 0..values_data.len() {
    for (val, start, end) in values_data.get(row).unwrap().iter(){
        match determine_valid_numbers(row, *start, *end, matrix.clone()) {
            true => sum += val,
            false => (),
        }
    }
  }
  sum
}

// Main function for day 3
pub fn main() {
  //filenames for input
  let filename = "src/day_03/day_03_input.txt";
//   let filename = "src/day_03/test_01.txt";
  
  let mut matrix :Vec<Vec<i32>> = Vec::new();
  let mut values_data : Vec<Vec<(i32, i32, i32)>> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        // Get symbols' indices and values' indices for each row
          if let Ok(ip) = line {
            matrix.push(get_symbol_locations_row_i(ip.clone()));
            values_data.push(get_number_locations_row_i(ip))
          }
      }
      let sum = sum_valid_numbers(matrix, values_data);
      println!("The final sum is: {}", sum);
  }
}
