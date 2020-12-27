use crate::solution::Solution;

pub struct Day13 {
    departure: u32,
    buses: Vec<u32>,
}

impl Day13 {
    pub fn new(input: &Vec<&str>) -> Day13 {
        Day13 {
            departure: input[0].parse::<u32>().unwrap(),
            buses: input[1].split(",")
                .filter(|s| *s != "x")
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        }
    }
}

impl Solution for Day13 {
    fn part_1(&mut self) -> String {
        let mut timestamp = self.departure;
        loop {
            for bus in &self.buses {
                if timestamp % bus == 0 {
                    return (bus * (timestamp - self.departure)).to_string();
                }
            }
            timestamp = timestamp + 1;
        }
    }

    fn part_2(&mut self) -> String {
        "0".to_string()
    }
}
