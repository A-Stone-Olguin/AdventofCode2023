use crate::helper_functions::io::*;

#[derive(Debug)]

struct Location {
    row : u32,
    col : u32,
    symbol : char,
}

fn create_location(row : u32, col : u32, symbol : char) -> Location {
    Location {row, col, symbol}
}

// Returns either a tuple of the locations with the location of S, or just the locations
fn create_locations(line_string : String, row : u32) -> Result<(Location, Vec<Location>), Vec<Location>> {
    let mut locations = vec![];
    let mut s_col_loc = 0;
    let mut has_s = false;
    for (col, c) in line_string.chars().enumerate() {
        let location = create_location(row, col as u32, c);
        if c == 'S' {
            has_s = true;
            s_col_loc = col as u32;
        }
        locations.push(location);
    }   
    if has_s {
        let s_location = create_location(row, s_col_loc, 'S');
        return Ok( (s_location, locations) );
    }

    Err(locations)
}

// Determine what the starting symbol should be for the start
fn determine_start_pipe(start_loc : &Location, loc_matrix : &Vec<Vec<Location>>) -> char {
    let start_row = start_loc.row as usize;
    let start_col = start_loc.col as usize;
    let above = loc_matrix[start_row-1][start_col].symbol;
    let below = loc_matrix[start_row+1][start_col].symbol;
    let left = loc_matrix[start_row][start_col-1].symbol;
    let right = loc_matrix[start_row][start_col+1].symbol;

    let below_vertical = below == 'J' || below == '|' || below == 'L';
    let above_vertical = above == '7' || above == '|' || above == 'F';
    let left_horizontal = left == 'F' || left == '-' || left == 'L';
    let right_horizontal = right == 'J' || right == '-' || right == '7';

    // Arbitrary start symbol chosen
    let mut start_symbol = 'F';

    // This will end up being a mess
    if below_vertical && above_vertical {
        start_symbol = '|';
    }
    else if below_vertical && left_horizontal {
        start_symbol = '7';
    }
    else if below_vertical && right_horizontal {
        start_symbol = 'F';
    }
    else if above_vertical && left_horizontal {
        start_symbol = 'J';
    }
    else if above_vertical && right_horizontal {
        start_symbol = 'L';
    }
    else if left_horizontal && right_horizontal {
        start_symbol = '-';
    }
    else {
        println!("Uh oh! An error occurred in determining the start symbol!");
    }

    start_symbol
}

// Counts the number of steps by walking the correct directions
fn walk_steps(curr_loc : &Location, dir_from : char, loc_matrix : &Vec<Vec<Location>>) -> u32 {
    if curr_loc.symbol == 'S' {
        return 1;
    }

    let curr_row = curr_loc.row as usize;
    let curr_col = curr_loc.col as usize;
    // Determine which direction to go from where it came from
    let mut next_dir_from = 'b';
    let mut next_location = curr_loc;
    if curr_loc.symbol == '|' {
        if dir_from == 'b' {
            next_dir_from = 'b';
            next_location = &loc_matrix[curr_row-1][curr_col];
        }
        else {
            next_dir_from = 'a';
            next_location = &loc_matrix[curr_row+1][curr_col];
        }
    }
    else if curr_loc.symbol == '-' {
        if dir_from == 'l' {
            next_dir_from = 'l';
            next_location = &loc_matrix[curr_row][curr_col+1];
        }
        else {
            next_dir_from = 'r';
            next_location = &loc_matrix[curr_row][curr_col-1];
        }
    }
    else if curr_loc.symbol == 'F' {
        if dir_from == 'r' {
            next_dir_from = 'a';
            next_location = &loc_matrix[curr_row+1][curr_col];
        }
        else {
            next_dir_from = 'l';
            next_location = &loc_matrix[curr_row][curr_col+1];
        }
    }
    else if curr_loc.symbol == 'L' {
        if dir_from == 'a' {
            next_dir_from = 'l';
            next_location = &loc_matrix[curr_row][curr_col+1];
        }
        else {
            next_dir_from = 'b';
            next_location = &loc_matrix[curr_row-1][curr_col];
        }
    }
    else if curr_loc.symbol == '7' {
        if dir_from == 'l' {
            next_dir_from = 'a';
            next_location = &loc_matrix[curr_row+1][curr_col];
        }
        else {
            next_dir_from = 'r';
            next_location = &loc_matrix[curr_row][curr_col-1];
        }
    }
    else if curr_loc.symbol == 'J' {
        if dir_from == 'a' {
            next_dir_from = 'r';
            next_location = &loc_matrix[curr_row][curr_col-1];
        }
        else {
            next_dir_from = 'b';
            next_location = &loc_matrix[curr_row-1][curr_col];
        }
    }
    1 + walk_steps(next_location, next_dir_from, loc_matrix)
}

fn start_walk(start_loc : &Location, start_sym : char, loc_matrix : &Vec<Vec<Location>>) -> u32 {
    // If given the chance, we choose the top or right first, 
    let start_row = start_loc.row as usize;
    let start_col = start_loc.col as usize;

    let mut next_loc = start_loc;
    let mut next_from_dir = 'F';

    if start_sym == '|' || start_sym == 'J' || start_sym == 'L' {
        next_from_dir = 'b';
        next_loc = &loc_matrix[start_row-1][start_col];
    }
    else if start_sym == 'F' || start_sym == '-' {
        next_from_dir = 'l';
        next_loc = &loc_matrix[start_row][start_col+1];
    }
    // We choose to go left
    else if start_sym == '7' {
        next_from_dir = 'r';
        next_loc = &loc_matrix[start_row][start_col-1];
    }
    else {
        println!("Invalid start symbol at beginning of walk!");
    }

    // Get the ceiling of the integer division
    (walk_steps(next_loc, next_from_dir, loc_matrix) + 1)/2

}

// Main function for day 10
pub fn main() {
    // filenames for input
    let filename = "src/day_10/day_10_input.txt";
    // let filename = "src/day_10/test_01.txt";

    let mut locations_matrix : Vec<Vec<Location>> = vec![];

    // Placeholder for the location of S
    let mut s_location = create_location(0, 0, 'S');

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (row, line) in lines.enumerate() {
            if let Ok(ip) = line {
                match create_locations(ip, row as u32) {
                    Ok((s_loc, locations)) => {s_location = s_loc; locations_matrix.push(locations)},
                    Err(locations) => locations_matrix.push(locations),
                };
            }
        }
        let start_sym = determine_start_pipe(&s_location, &locations_matrix);

        println!("The longest path is: {}", start_walk(&s_location, start_sym, &locations_matrix));
    }
}
