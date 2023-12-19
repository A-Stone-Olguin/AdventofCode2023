use crate::helper_functions::io::*;
use regex::Regex;
use std::vec;

// Get the seeds, the first line of the input
fn get_seeds(line_string : String) -> Vec<u32> {
    let re = match Regex::new(r"(\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut seeds : Vec<u32> = vec![];
    for (_, [seed]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        seeds.push(seed.parse::<u32>().unwrap());
    }
    seeds
}

// Make the mapping
fn make_map(line_string : String, map_info : &mut Vec<Vec<(u32, u32, u32)>>) -> Vec<Vec<(u32, u32, u32)>>{
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
            let len_int = match length.parse::<u32>() {
                Ok(l) => l,
                Err(msg) => panic!("Error: {}", msg),
            };
            let range_low_int = match range_lower.parse::<u32>() {
                Ok(l) => l,
                Err(msg) => panic!("Error: {}", msg),
            };
            let domain_low_int = match domain_lower.parse::<u32>() {
                Ok(l) => l,
                Err(msg) => panic!("Error: {}", msg),
            };
            m_i.push((domain_low_int, range_low_int, len_int));
        }
    }
    map_info.to_vec()
}

// Determine the location for a given seed
fn seed_location(seed : u32, map_info : Vec<Vec<(u32, u32, u32)>>) -> u32 {
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

// Main function for day 5
pub fn main() {
  //filenames for input
  let filename = "src/day_05/day_05_input.txt";
//   let filename = "src/day_05/test_01.txt";

  let mut first_line = true;
  let mut seeds : Vec<u32> = vec![];
  let mut map_info : Vec<Vec<(u32, u32, u32)>> = vec![];
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

      // Now we have the line info, let's determine where each seed goes!
      let mut locations:Vec<u32> = vec![];
      for seed in seeds.iter() {
        locations.push(seed_location(*seed, map_info.clone()));
      }
      println!("The locations are: {:?}", locations);
      println!("The smallest location is: {:?}", locations.iter().min().unwrap());
  }
}
