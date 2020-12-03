pub use self::day1::Day1;
pub use self::day2::Day2;

pub trait Solution {
    fn part_1(&self, input: &Vec<&str>) -> String;
    fn part_2(&self, input: &Vec<&str>) -> String;
}

pub mod day1;
pub mod day2;
