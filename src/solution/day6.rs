use crate::solution::Solution;

use std::convert::TryInto;
use std::collections::HashMap;

struct ResponseGroup {
    responses: Vec<Vec<char>>,
}

impl ResponseGroup {
    fn response_counts(&self) -> HashMap<&char, u32> {
        let mut response_counts = HashMap::new();
        for response in &self.responses {
            for c in response {
                if response_counts.contains_key(c) {
                    response_counts.insert(c, response_counts.get(c).unwrap() + 1);
                }
                else {
                    response_counts.insert(c, 1);
                }
            }
        }
        response_counts
    }

    fn unique_responses(&self) -> usize {
        self.response_counts().keys().len()
    }

    fn common_responses(&self) -> usize {
        self.response_counts().values()
            .filter(|count| **count == self.responses.len().try_into().unwrap())
            .count()
    }
}

pub struct Day6 {
    response_groups: Vec<ResponseGroup>,
}

impl Day6 {
    pub fn new(input: &Vec<&str>) -> Day6 {
        let mut response_groups = vec![];
        let mut responses = vec![];
        for (idx, line) in input.iter().enumerate() {
            if !line.is_empty() {
                responses.push(line.chars().collect());
            }
            if line.is_empty() || idx == input.len() - 1 {
                response_groups.push(
                    ResponseGroup {
                        responses: responses.clone()
                    }
                );
                responses.clear();
            }
        }
        Day6 {
            response_groups: response_groups,
        }
    }
}

impl Solution for Day6 {
    fn part_1(&mut self) -> String {
        self.response_groups.iter()
            .map(|r| r.unique_responses())
            .fold(0, |a, b| a + b)
            .to_string()
    }

    fn part_2(&mut self) -> String {
        self.response_groups.iter()
            .map(|r| r.common_responses())
            .fold(0, |a, b| a + b)
            .to_string()
    }
}
