extern crate clap;
use clap::App;

use std::fs;
use std::process;

pub mod solution;

use solution::day1::Day1;
use solution::Solution;

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

    let input: Vec<&str> = input_text.split("\n").filter(|s| !s.is_empty()).collect();

    let solution = match day {
        1 => Day1 {},
        _ => {
            println!("Solution not available for day {}", day);
            process::exit(1);
        }
    };

    println!("part 1: {}", solution.part_1(&input));
    println!("part 2: {}", solution.part_2(&input));
}
