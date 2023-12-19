use crate::helper_functions::io::*;
use regex::Regex;

// Get the seeds, the first line of the input
fn get_numbers(line_string : String) -> u64 {
    let re = match Regex::new(r"(\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut combined_number = String::from("");
    for (_, [number]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        combined_number = combined_number.to_owned() +  number;
    }
    combined_number.parse::<u64>().unwrap()
}

// Determines if a hold can make the distance
fn can_beat_score(time_held : &u64, total_time : &u64, distance : &u64) -> bool {
    let time_left = total_time - time_held;
    let distance_traveled = time_left * time_held;
    distance_traveled > *distance
}

fn get_winning_holding_times(time : u64, distance : u64) -> u64 {
    // Get the paired time/distance pair
    let mut count = 0;
    for time_held in 0..time {
        if can_beat_score(&time_held, &time, &distance) {
            count += 1;
        }
    }
    count
}

// Main function for day 6
pub fn main() {
    // filenames for input
    let filename = "src/day_06/day_06_input.txt";
    // let filename = "src/day_06/test_02.txt";

    let mut time = 0;
    let mut distance= 0;

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if i == 0 {
                    time = get_numbers(ip);
                }
                else if i == 1 {
                    distance = get_numbers(ip);
                }
                else {
                    panic!("ERROR! Should not be more than two lines!")
                }
            }
        }
        println!("The final result is: {}", get_winning_holding_times(time, distance))

    }
}
