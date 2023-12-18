use std::vec;

use crate::helper_functions::io::*;

use regex::Regex;

// Function that determines each symbol's location
// Should only use the first part of the tuple, could be fixed later on
fn get_symbol_locations_row_i(line_string : String) -> Vec<(i32, i32, i32)> {
    // Capture the symbols in the row
    let re = match Regex::new(r"([^\d\.])") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut indices = vec![];
    // Push each capture's location to the indices matrix
    for capture in re.captures_iter(&line_string) {
        indices.push((capture.get(0).unwrap().start() as i32, 0 , 0));
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

// Grab the previous row, current row, and next row
fn get_subset_rows(row_num : usize, values_data : Vec<Vec<(i32, i32, i32)>>) -> Vec<Vec<(i32, i32, i32)>> {
    let mut sub_matrix: Vec<Vec<(i32, i32, i32)>> = vec![];

    // Grab the lower_bounds and upper_bounds based on if they exist
    let lower_bound = match row_num {
        0 => row_num,
        _ => row_num -1,
    };
    let upper_bound = match row_num == values_data.len() - 1 {
        true => row_num - 1,
        false => row_num,
    };
    // Push each row to the submatrix
    for i in lower_bound..upper_bound+2 {
        let ith_row = match values_data.get(i) {
            Some (row) => row,
            None => panic!("Error on grabbing index!"),
        };
        sub_matrix.push(ith_row.to_vec());
    }
    sub_matrix
}

// Multiply two gear numbers if they are valid
fn multiply_gear_numbers(row_num : usize, col_num : i32, values_data : Vec<Vec<(i32, i32, i32)>>) -> i32 {
    let mut gear_numbers : Vec<i32> = vec![];
    let sub_values_data = get_subset_rows(row_num, values_data);
    for (_vals_row_num, sub_row) in sub_values_data.iter().enumerate() {
        for (value, start_col, end_col) in sub_row.iter() {
            for col in start_col-1..end_col+2 {
                if col == col_num {
                    gear_numbers.push(*value);
                }
            }
        }
    }
    if gear_numbers.len() >= 3 {
        println!("Three numbers on a gear!");
        return 0;
    }

    let first = match gear_numbers.get(0) {
        Some(f) => f,
        None => &0,
    };
    let second = match gear_numbers.get(1) {
        Some(s) => s,
        None => &0,
    };
    first * second
}

fn sum_gear_numbers(matrix : Vec<Vec<(i32, i32, i32)>>, values_data : Vec<Vec<(i32, i32, i32)>>) -> i32 {        
    let mut sum = 0;

    for (row_num, row) in matrix.iter().enumerate() {
        for (col_num, _, _) in row.iter() {
            sum += multiply_gear_numbers(row_num, *col_num, values_data.clone());
        }
    }
    sum
}

// Main function for day 3
pub fn main() {
  //filenames for input
  let filename = "src/day_03/day_03_input.txt";
//   let filename = "src/day_03/test_02.txt";
  
  let mut matrix :Vec<Vec<(i32, i32, i32)>> = Vec::new();
  let mut values_data : Vec<Vec<(i32, i32, i32)>> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(ip) = line {
            matrix.push(get_symbol_locations_row_i(ip.clone()));
            values_data.push(get_number_locations_row_i(ip))
          }
      }
      let sum = sum_gear_numbers(matrix, values_data);
      println!("The final sum is: {}", sum);
  }
}
