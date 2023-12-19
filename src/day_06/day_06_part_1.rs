use crate::helper_functions::io::*;
use regex::Regex;
use std::vec;

// Get the seeds, the first line of the input
fn get_numbers(line_string : String) -> Vec<u32> {
    let re = match Regex::new(r"(\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut numbers : Vec<u32> = vec![];
    for (_, [number]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        numbers.push(number.parse::<u32>().unwrap());
    }
    numbers
}

// Determines if a hold can make the distance
fn can_beat_score(time_held : &u32, total_time : &u32, distance : &u32) -> bool {
    let time_left = total_time - time_held;
    let distance_traveled = time_left * time_held;
    distance_traveled > *distance
}

fn get_winning_holding_times(times : Vec<u32>, distances : Vec<u32>) -> u32 {
    // Get the paired time/distance pair
    let mut counts = vec![];
    for i in 0..times.len() {
        let mut count = 0;
        let time = times[i];
        let distance = match distances.get(i) {
            Some(d) => d,
            None => panic!("Non-paired distances and times!"),
        };
        for time_held in 0..time {
            if can_beat_score(&time_held, &time, distance) {
                count += 1;
            }
        }
        counts.push(count)
    }
    let mut product = 1;
    for count in counts {
        product *= count;
    }
    product

}

// Main function for day 6
pub fn main() {
    // filenames for input
    let filename = "src/day_06/day_06_input.txt";
    // let filename = "src/day_06/test_01.txt";

    let mut times = vec![];
    let mut distances = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if i == 0 {
                    times = get_numbers(ip);
                }
                else if i == 1 {
                    distances = get_numbers(ip);
                }
                else {
                    panic!("ERROR! Should not be more than two lines!")
                }
            }
        }
        println!("The final result is: {}", get_winning_holding_times(times, distances))

    }
}
