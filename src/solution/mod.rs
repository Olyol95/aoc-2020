pub use self::day1::Day1;
pub use self::day2::Day2;
pub use self::day3::Day3;
pub use self::day4::Day4;
pub use self::day5::Day5;
pub use self::day6::Day6;
pub use self::day7::Day7;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub trait Solution {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}
