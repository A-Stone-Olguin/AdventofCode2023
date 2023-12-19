use crate::helper_functions::io::*;
use regex::Regex;

// Function to get the initial sequence
fn get_sequence(line_string : String) -> Vec<i32> {
    let re = match Regex::new(r"(\-?\d+)") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };
    let mut sequence = vec![];
    for (_, [number]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        sequence.push(number.parse::<i32>().unwrap());
    }
    
    sequence
}

// Function to determine whether every element is 0 of a vector
fn is_all_zero(sequence : &Vec<i32>) -> bool {
    for val in sequence {
        if *val != 0 {
            return false;
        }
    }
    true
}

// Generate the next histories
fn generate_histories(sequence : Vec<i32>) -> Vec<Vec<i32>> {
    let mut histories = vec![];
    let mut current_seq = sequence;

    // Here was my code: It didn't work for some reason?
    while !is_all_zero(&current_seq) {
        let mut new_seq = vec![];
        for i in 0..current_seq.len() - 1 {
            new_seq.push(&current_seq[i+1] - &current_seq[i] )
        }
        histories.push(new_seq.to_vec());
        current_seq = new_seq;
    }

    histories
}

// Get the next value for each initial sequence
fn get_next_value(sequence : &Vec<i32>) -> i32 {
    let histories = generate_histories(sequence.to_vec());
    // println!("Seq: {:?}", sequence);
    // println!("Histories: {:?}\n", histories);

    // Update each history's last value
    let mut last_val = 0;
    for history in histories.iter().rev() {
        // Get the histories' last value
        let hist_last_val = history.iter().last().unwrap();
        last_val+= hist_last_val;
    }
    // Add the final last value to the original sequence
    last_val += sequence.iter().last().unwrap();
    last_val
}

// Main function for day 9
pub fn main() {
    // filenames for input
    let filename = "src/day_09/day_09_input.txt";
    // let filename = "src/day_09/test_01.txt";

    let mut sequences : Vec<Vec<i32>> = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                sequences.push(get_sequence(ip));
            }
        }
        // Go through and get each sequence's value to add
        let sum : i32 = sequences.iter().map(|seq| get_next_value(seq)).sum();
        println!("The final sum is: {sum}")
    }
}
