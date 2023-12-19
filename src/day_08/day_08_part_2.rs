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


// Determine whether every current path value ends in a Z
fn all_end_in_z(current_path : &Vec<String>) -> bool {
    for path in current_path {
        if !ends_with_char(path, 'Z') {
            return false;
        }
    }
    true
}

// Walk a set of start points at the same time
fn walk_all_paths(pattern : String, paths:  &HashMap<String, (String, String)>, start_paths: Vec<String>) -> i32 {
    let mut number_steps = 0;
    let mut current_path = start_paths;

    // Get pattern as a vector to repeat if possible
    let pattern_vec: Vec<char> = pattern.chars().collect();
    let pattern_length = pattern_vec.len();
    while !all_end_in_z(&current_path) {
        // println!("Current path: {:?}", current_path);
        let path_choice = pattern_vec[number_steps % pattern_length];
        for path in current_path.iter_mut() {
            *path = match path_choice {
                'L' => paths[&path.to_string()].0.to_string(),
                'R' => paths[&path.to_string()].1.to_string(),
                _ => panic!("ERROR! Path uses not left and right values!"),
            };
        }
        number_steps+=1;


    }
    
    number_steps as i32
}

// Lets the user know whether the string ends with a given char
fn ends_with_char(s : &String, c : char) -> bool {
    c == s.chars().last().unwrap()
}


// Main function for day 8
pub fn main() {
    // filenames for input
    let filename = "src/day_08/day_08_input.txt";
    // let filename = "src/day_08/test_02.txt";

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
        // Get all start and end points
        let mut start_points = vec![];
        for key in paths.keys() {
            if ends_with_char(key, 'A') {
                start_points.push(key.to_string());
            }
        }
        println!("The number of steps were: {}", walk_all_paths(pattern, &paths, start_points));

    }
}
