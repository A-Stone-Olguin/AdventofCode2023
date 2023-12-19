use crate::helper_functions::io::*;
use regex::Regex;
use std::collections::HashMap;

fn get_path_info(line_string : String, hm : &mut HashMap<String, (String, String)>) {
    let re = match Regex::new(r"(\w{3})") {
        Ok(r) => r,
        Err(msg) => panic!("Error: {}", msg),
    };

    let mut paths = vec![];
    for (_, [path]) in re.captures_iter(&line_string).map(|c| c.extract()) {
        paths.push(path);
    }
    hm.insert(paths[0].to_string(), (paths[1].to_string(), paths[2].to_string()) );
}

fn walk_path(pattern : String, paths: &HashMap<String, (String, String)>) -> i32 {
    let mut number_steps = 0;
    let mut current = String::from("AAA");

    // Get pattern as a vector to repeat if possible
    let pattern_vec: Vec<char> = pattern.chars().collect();
    let pattern_length = pattern_vec.len();
    while current != "ZZZ" {
        let path_choice = pattern_vec[number_steps % pattern_length];
        // Choose left or right path
        current = match path_choice {
            'L' => paths[&current.to_string()].0.to_string(),
            'R' => paths[&current.to_string()].1.to_string(),
            _ => panic!("ERROR! Path uses not left and right values!"),
        };
        number_steps+=1;
    }
    number_steps as i32
}


// Main function for day 8
pub fn main() {
    // filenames for input
    let filename = "src/day_08/day_08_input.txt";
    // let filename = "src/day_08/test_01.txt";

    let mut pattern = String::from("");
    let mut paths : HashMap<String, (String, String)> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if i == 0 {
                    pattern = ip.trim().to_string();
                }
                else if i > 1 {
                    get_path_info(ip, &mut paths);
                }
            }
        }
        println!("The number of steps were: {}", walk_path(pattern, &paths));

    }
}
