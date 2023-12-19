use crate::helper_functions::io::*;
use regex::Regex;
use std::vec;

// Get the seeds, the first line of the input
fn get_seeds(line_string : String) -> Vec<(u64, u64)> {
    let re = match Regex::new(r"(\d+)\s*(\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut seeds : Vec<(u64, u64)> = vec![];
    for (_, [seed, number]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        let seed_int = seed.parse::<u64>().unwrap();
        let len_int = number.parse::<u64>().unwrap();
        seeds.push((seed_int, len_int));
    }
    seeds
}

// Make the mapping
fn make_map(line_string : String, map_info : &mut Vec<Vec<(u64, u64, u64)>>) -> Vec<Vec<(u64, u64, u64)>>{
    let re = match Regex::new(r"to") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut m_i = map_info.clone();
    if re.is_match(&line_string) || line_string.len() == 0 {
        m_i.push(Vec::new());
        return m_i;
    }
    else {
        let m_i = match map_info.last_mut() {
            Some(l) => l,
            None => panic!("Uh oh, it's empty!"),
        };
        let re = match Regex::new(r"(\d+)\s*(\d+)\s*(\d+)") {
            Ok(r) => r,
            Err(msg) => panic!("Error: {}", msg),
        };
        for (_, [range_lower, domain_lower, length]) in re.captures_iter(&line_string).map(|c| c.extract()) {
            let len_int = match length.parse::<u64>() {
                Ok(l) => l,
                Err(msg) => panic!("Error: {}", msg),
            };
            let range_low_int = match range_lower.parse::<u64>() {
                Ok(l) => l,
                Err(msg) => panic!("Error: {}", msg),
            };
            let domain_low_int = match domain_lower.parse::<u64>() {
                Ok(l) => l,
                Err(msg) => panic!("Error: {}", msg),
            };
            m_i.push((domain_low_int, range_low_int, len_int));
        }
    }
    map_info.to_vec()
}

// Determine the location for a given seed
fn seed_location(seed : u64, map_info : &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut map_location = seed;
    for m_i in map_info.iter() {
        if !m_i.is_empty() {
            for row in m_i.iter() {
                let domain_low = row.0;
                let range_low = row.1;
                let len_int = row.2;
                // Determine if location is in this range
                if map_location >= domain_low && map_location < domain_low + len_int {
                    let dist = map_location - domain_low;
                    map_location = range_low + dist;
                    break;
                }
            }
        }
    }
    map_location
}

// Binary search function to determine the smallest location value
fn binary_search(left : u64, right : u64, map_info : &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    if left == right {
        return seed_location(left, map_info);
    }
    else if right - left == 1 {
        let left_result = seed_location(left, map_info);
        let right_result = seed_location(right, map_info);
        match left_result < right_result {
            true => return left_result,
            false => return right_result,
        }
    }
    let midpoint = (left + right)/2;

    let left_result = seed_location(left, map_info);
    let mid_result = seed_location(midpoint, map_info);
    let (new_left, new_right) = match left_result < mid_result {
        true => (midpoint, right),
        false => (left, midpoint),
    };
    binary_search(new_left, new_right, map_info)
}

// Main function for day 5
pub fn main() {
  //filenames for input
  let filename = "src/day_05/day_05_input.txt";
//   let filename = "src/day_05/test_02.txt";

  let mut first_line = true;
  let mut seeds : Vec<(u64, u64)> = vec![];
  let mut map_info : Vec<Vec<(u64, u64, u64)>> = vec![];
  if let Ok(lines) = read_lines(filename) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(ip) = line {
            if first_line {
                first_line = false;
                seeds = get_seeds(ip);
            }
            else {
              map_info = make_map(ip, &mut map_info);
            }
          }
      }
      // Find starting location (where the minimum is between the values)
      let mut locations = vec![];
      for (seed, len) in seeds.iter() {
        let start_seed_location = *seed;
        let end_seed_location = *seed + *len ;
        let result = binary_search(start_seed_location, end_seed_location, &map_info);
        locations.push(result);
      }
      println!("Locations: {:?}", locations);
      println!("The smallest location is: {:?}", locations.iter().min().unwrap());
  }
}
