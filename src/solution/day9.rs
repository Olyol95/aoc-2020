use crate::solution::Solution;

use std::collections::VecDeque;

pub struct Day9 {
    values: Vec<u64>,
}

impl Day9 {
    pub fn new(input: &Vec<&str>) -> Day9 {
        Day9 {
            values: input.iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
        }
    }

    fn first_invalid_val(&mut self) -> u64 {
        let mut buffer: VecDeque<u64> = VecDeque::new();
        for (idx, val) in self.values.iter().enumerate() {
            if idx >= 25 {
                let mut valid = false;
                'outer: for tortoise in 0..buffer.len() - 1 {
                    let x = buffer[tortoise];
                    for hare in tortoise + 1..buffer.len() {
                        let y = buffer[hare];
                        if x + y == *val {
                            valid = true;
                            break 'outer;
                        }
                    }
                }
                if !valid {
                    return *val;
                }
                else {
                    buffer.pop_back();
                }
            }
            buffer.push_front(*val);
        }
        0
    }

    fn contiguous_range_summing_to(&mut self, val: u64) -> Vec<u64> {
        for start_idx in 0..self.values.len() - 1 {
            let mut range = vec![self.values[start_idx]];
            for end_idx in start_idx + 1..self.values.len() {
                range.push(self.values[end_idx]);
                let sum = range.iter().fold(0, |a, b| a + b);
                if sum == val {
                    return range;
                }
                else if sum > val {
                    break;
                }
            }
        }
        vec![]
    }
}

impl Solution for Day9 {
    fn part_1(&mut self) -> String {
        self.first_invalid_val().to_string()
    }

    fn part_2(&mut self) -> String {
        let val = self.first_invalid_val();
        let range = self.contiguous_range_summing_to(val);
        (range.first().unwrap() + range.last().unwrap()).to_string()
    }
}
