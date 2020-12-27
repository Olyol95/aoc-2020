use crate::solution::Solution;

use regex::Regex;

use std::collections::HashMap;
use std::convert::TryInto;

struct U36 {
    bits: [bool; 36],
}

impl U36 {
    pub fn from_str(input: &str) -> U36 {
        let mut bits: [bool; 36] = [false; 36];
        let mut val = input.parse::<u64>().unwrap();
        for idx in (0..36).rev() {
            let power = 2_u64.pow(idx.try_into().unwrap());
            if val >= power {
                bits[35 - idx] = true;
                val = val - power;
            }
        }
        U36 {
            bits: bits,
        }
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        self.bits[idx] = value;
    }

    pub fn to_u64(&self) -> u64 {
        let mut val: u64 = 0;
        for idx in 0..36 {
            if self.bits[35 - idx] {
                val = val + (2_u64.pow(idx.try_into().unwrap()));
            }
        }
        val
    }

    pub fn to_string(&self) -> String {
        self.bits.iter()
            .map(|b| match b {
                true  => '1',
                false => '0',
            })
            .collect()
    }
}

struct Instruction {
    variable: String,
    expression: String,
}

pub struct Day14 {
    memory: HashMap<u16, U36>,
    bitmask: [char; 36],
    instr: Vec<Instruction>,
}

impl Day14 {
    pub fn new(input: &Vec<&str>) -> Day14 {
        let mut instr = vec![];
        for line in input {
            let parts: Vec<&str> = line.split(" = ").collect();
            instr.push(Instruction {
                variable: parts[0].to_string(),
                expression: parts[1].to_string(),
            });
        }
        Day14 {
            memory: HashMap::new(),
            bitmask: ['X'; 36],
            instr: instr,
        }
    }

    fn run(&mut self) {
        let mem_regex = Regex::new(r"^mem\[([^\]]+)\]").unwrap();
        for i in &self.instr {
            if i.variable == "mask" {
                for (idx, c) in i.expression.chars().enumerate() {
                    self.bitmask[idx] = c;
                }
            }
            else if mem_regex.is_match(&i.variable) {
                let captures = mem_regex.captures(&i.variable).unwrap();
                let mut val = U36::from_str(&i.expression);
                for idx in 0..36 {
                    match self.bitmask[idx] {
                        '0' => { val.set(idx, false) },
                        '1' => { val.set(idx, true)  },
                        _ => {},
                    }
                }
                let address = captures[1].parse::<u16>().unwrap();
                self.memory.insert(address, val);
            }
            else {
                panic!(
                    "Encountered unknown instruction: {} = {}",
                    i.variable,
                    i.expression
                );
            }
        }
    }
}

impl Solution for Day14 {
    fn part_1(&mut self) -> String {
        self.run();
        self.memory.values()
            .map(|v| v.to_u64())
            .fold(0, |a,b| a + b)
            .to_string()
    }

    fn part_2(&mut self) -> String {
        "0".to_string()
    }
}
