extern crate clap;
use clap::App;

use std::fs;
use std::process;

use aoc_2020::solution::*;

fn main() {
    let opts = App::new("Advent of Code Solutions 2020")
        .author("Oliver Youle")
        .args_from_usage(
            "-d, --day=<DAY_OF_MONTH> 'The number of the solution to run'
             -i, --input=<FILE_PATH>  'Path to the problem input'",
        )
        .get_matches();

    let day: u8 = match opts.value_of("day").unwrap().parse::<u8>() {
        Ok(n) => n,
        Err(_e) => {
            println!("The provided day is not a valid");
            process::exit(1);
        }
    };

    let filename = opts.value_of("input").unwrap();
    let input_text = match fs::read_to_string(filename) {
        Ok(i) => i,
        Err(e) => {
            println!("Error reading from input file: {}", e);
            process::exit(1);
        }
    };

    let input = input_text.lines().collect();

    let mut solution = match day {
        1  => Box::new( Day1::new(&input) ) as Box<dyn Solution>,
        2  => Box::new( Day2::new(&input) ) as Box<dyn Solution>,
        3  => Box::new( Day3::new(&input) ) as Box<dyn Solution>,
        4  => Box::new( Day4::new(&input) ) as Box<dyn Solution>,
        5  => Box::new( Day5::new(&input) ) as Box<dyn Solution>,
        6  => Box::new( Day6::new(&input) ) as Box<dyn Solution>,
        7  => Box::new( Day7::new(&input) ) as Box<dyn Solution>,
        8  => Box::new( Day8::new(&input) ) as Box<dyn Solution>,
        9  => Box::new( Day9::new(&input) ) as Box<dyn Solution>,
        10 => Box::new( Day10::new(&input) ) as Box<dyn Solution>,
        11 => Box::new( Day11::new(&input) ) as Box<dyn Solution>,
        12 => Box::new( Day12::new(&input) ) as Box<dyn Solution>,
        13 => Box::new( Day13::new(&input) ) as Box<dyn Solution>,
        _ => {
            println!("Solution not available for day {}", day);
            process::exit(1);
        }
    };

    println!("part 1: {}", solution.part_1());
    println!("part 2: {}", solution.part_2());
}
