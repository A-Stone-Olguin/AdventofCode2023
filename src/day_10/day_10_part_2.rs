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

// Walk each location, marking it as a pipe location
fn walk_steps(curr_loc : &Location, dir_from : char, loc_matrix : &Vec<Vec<Location>>, marked_locations : &mut Vec<Vec<u32>>) {

    let curr_row = curr_loc.row as usize;
    let curr_col = curr_loc.col as usize;
    marked_locations[curr_row][curr_col] = curr_loc.col + 1;

    if curr_loc.symbol == 'S' {
        return;
    }
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
    walk_steps(next_location, next_dir_from, loc_matrix, marked_locations)
}

fn start_walk(start_loc : &Location, start_sym : char, loc_matrix : &Vec<Vec<Location>>,marked_locations : &mut Vec<Vec<u32>>)  {
    // If given the chance, we choose the top or right first, 
    let start_row = start_loc.row as usize;
    let start_col = start_loc.col as usize;
    marked_locations[start_row][start_col] = start_loc.col;

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

    // Mark the other locations
    walk_steps(next_loc, next_from_dir, loc_matrix, marked_locations)

}

// Given a row with marked locations, find each interval
// fn window_pairs(row : &Vec<u32>) {
//     let pos_mark : Vec<u32> = row.iter().filter(|&&n| n != 0).cloned().collect();
//     pos_mark.windows(2)
// }

// // Return the smallest postive minimum and maximum
// fn start_end_marked_row(row : & Vec<u32>) -> (u32, u32) {
//     let max = row.iter().max().unwrap();
//     // Find the second smallest element
//     let mut min = max;
//     for element in row {
//         if element < min && element != &0 {
//             min = element;
//         }
//     }
//     // Subtract one to get the locations since we added one earlier
//     (*min, *max)
// }

// Determine if the nonmarked locations are bounded left-to-right and up
fn count_bounded(loc_matrix : &Vec<Vec<Location>>, marked_locations : &Vec<Vec<u32>>) -> u32 {

    let mut interior_locs = vec![];
    // Check if bounded left to right
    for row in 0..loc_matrix.len() {
        let pos_mark : Vec<&u32> = marked_locations[row].iter().filter(|&&n| n != 0).collect();

        for col in 0..loc_matrix[row].len() {
            let marked_value = &marked_locations[row][col];
            let mut iter = pos_mark.chunks(2);
            while let Some(&[first, second]) = iter.next() {
            // for &[first, second] in pos_mark.windows(2) {
                if (col as u32 + 1) > *first && (col as u32 + 1) < *second && marked_value == &0 {
                    interior_locs.push((row, col));
                    break;
                }
            }
        }

        // for col in 0..loc_matrix[row].len() {
        //     let marked_value = &marked_locations[row][col];
        //     if (col as u32 + 1) > min && (col as u32 + 1) < max && marked_value == &0 {
        //         interior_locs.push((row, col));
        //     }
        // }
    }
    
    // Check bounded top to bottom
    let mut count = 0;
    println!("{:?}", interior_locs);
    for (row, col) in interior_locs {
        // Make a vector of columns
        let mut columns = vec![];
        for (i, m_row) in marked_locations.iter().enumerate() {
            match m_row[col] == 0 {
                true => (),
                false => columns.push(i as u32 + 1),
            }
        }
        // let new_cols = marked_locations.iter().filter(|&roww| roww.iter()
        //         .filter(|&coll| col ))
        // Iterate through each chunk
        let mut iter = columns.chunks(2);
        while let Some([first, second]) = iter.next() {
            if &(row as u32 + 1) > first && &(row as u32 + 1) < second {
                count += 1;
                break;
            }
        }
    }
    count
}

// Main function for day 10
pub fn main() {
    // filenames for input
    // let filename = "src/day_10/day_10_input.txt";
    let filename = "src/day_10/test_02.txt";
    // let filename = "src/day_10/test_01.txt";

    let mut locations_matrix : Vec<Vec<Location>> = vec![];

    // This will be all of the pipe locations ordered by row and column
    let mut marked_matrix : Vec<Vec<u32>> = vec![];


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

        // Initialize marked locations as the rows and columns
        for row in &locations_matrix {
            let mut mark_row = vec![];
            for _ in row {
                mark_row.push(0);
            }
            marked_matrix.push(mark_row);
        }
        // Update the marked matrix
        start_walk(&s_location, start_sym, &locations_matrix, &mut marked_matrix);
        println!("The marked matrix is: {:?}", marked_matrix);

        println!("The number of interior is: {}", count_bounded(&locations_matrix, &marked_matrix))
    }
}
