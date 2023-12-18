use clap::{arg, command, value_parser};

use crate::day_01::*;
use crate::day_02::*;
use crate::day_03::*;
use crate::day_04::*;
use crate::day_05::*;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
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

        run_day_part(day_val, part_val)
}


// Function that cases based on the input to use a certain day's function.
fn run_day_part(day : &u8, part : &u8) {
    match day {
        1 => match part {
            1 => day_01_part_1::main(),
            2 => day_01_part_2::main(),
            _ => return,
        },
        2 => match part {
            1 => day_02_part_1::main(),
            2 => day_02_part_2::main(),
            _ => return,
        },
        3 => match part {
            1 => day_03_part_1::main(),
            2 => day_03_part_2::main(),
            _ => return,
        },
        4 => match part {
            1 => day_04_part_1::main(),
            2 => day_04_part_2::main(),
            _ => return,
        },
        5 => match part {
            1 => day_05_part_1::main(),
            2 => day_05_part_2::main(),
            _ => return,
        },
        _ => println!("Please provide a valid day/part combination."),
    }
}