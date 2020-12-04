pub use self::day1::Day1;
pub use self::day2::Day2;
pub use self::day3::Day3;

pub mod day1;
pub mod day2;
pub mod day3;

pub trait Solution {
    fn part_1(&self, input: &Vec<&str>) -> String;
    fn part_2(&self, input: &Vec<&str>) -> String;
}
