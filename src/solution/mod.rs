pub use self::day1::Day1;
pub use self::day2::Day2;
pub use self::day3::Day3;
pub use self::day4::Day4;
pub use self::day5::Day5;
pub use self::day6::Day6;
pub use self::day7::Day7;
pub use self::day8::Day8;
pub use self::day9::Day9;
pub use self::day10::Day10;
pub use self::day11::Day11;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;

pub trait Solution {
    fn part_1(&mut self) -> String;
    fn part_2(&mut self) -> String;
}
