use clap::{arg, command, value_parser};

use crate::day_01::*;
use crate::day_02::*;

pub mod day_01;
pub mod day_02;
pub mod helper_functions;

fn main() {
    let args = command!()
        .arg(
            arg!(--day <DAY> "The day's code you'd like to use")
            .required(false)
            .value_parser(value_parser!(u8))
        )
        .arg(
            arg!(--part <PART> "The part you would like to use for the day")
            .required(false)
            .value_parser(value_parser!(u8))
        )
        .get_matches();

        let day_pres = args.is_present("day");
        let part_pres = args.is_present("part");

        if !day_pres && part_pres || day_pres && !part_pres {
            println!("Please provide both a day AND a part to run =)");
            return;
        } 
        
        let day_val = match args.get_one::<u8>("day") {
            Some(val) => val,
            None => &1,
        };
        let part_val = match args.get_one::<u8>("part") {
            Some(val) => val,
            None => &1,
        };

        run_day_part_function(day_val, part_val)
}


// Function that cases based on the input to use a certain day's function.
fn run_day_part_function(day : &u8, part : &u8) {
    match day {
        1 => match part {
            1 => day_01_part_1::day_01_part_1(),
            2 => day_01_part_2::day_01_part_2(),
            _ => return,
        },
        2 => match part {
            1 => day_02_part_1::day_02_part_1(),
            2 => day_02_part_2::day_02_part_2(),
            _ => return,
        },
        _ => println!("Please provide a valid day/part combination."),
    }
}