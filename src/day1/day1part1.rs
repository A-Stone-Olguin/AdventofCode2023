use crate::helper_functions::io::*;

pub fn day_1_part_1() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./src/day1/hello.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}