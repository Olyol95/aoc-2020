use crate::solution::Solution;

use std::collections::HashMap;
use std::convert::TryInto;

pub struct Day15 {
    starting_numbers: Vec<u32>,
    history: HashMap<u32, (usize, usize)>,
    turn: usize,
}

impl Day15 {
    pub fn new(input: &Vec<&str>) -> Day15 {
        let starting_numbers = input.iter()
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Day15 {
            starting_numbers: starting_numbers,
            history: HashMap::new(),
            turn: 0,
        }
    }

    fn run(&mut self, turns: usize) -> u32 {
        let mut prev = 0;
        while self.turn < turns {
            prev = self.speak(prev);
        }
        self.history.clear();
        self.turn = 0;
        prev
    }

    fn speak(&mut self, prev: u32) -> u32 {
        self.turn = self.turn + 1;
        let mut val;
        if self.turn <= self.starting_numbers.len() {
            val = self.starting_numbers[self.turn - 1];
        }
        else {
            val = prev;
            let hist = self.get_history(&val);
            if hist.1 == 0 {
                val = 0;
            }
            else {
                val = (hist.0 - hist.1).try_into().unwrap();
            }
        }
        self.record(&val);
        val
    }

    fn get_history(&self, value: &u32) -> (usize, usize) {
        self.history.get(value).unwrap().clone()
    }

    fn record(&mut self, value: &u32) {
        if self.history.contains_key(value) {
            let hist = self.get_history(value);
            self.history.insert(*value, (self.turn, hist.0));
        }
        else {
            self.history.insert(*value, (self.turn, 0));
        }
    }
}

impl Solution for Day15 {
    fn part_1(&mut self) -> String {
        self.run(2020).to_string()
    }

    fn part_2(&mut self) -> String {
        self.run(30000000).to_string()
    }
}
