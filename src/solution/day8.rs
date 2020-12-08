use crate::solution::Solution;

use std::collections::HashMap;
use std::convert::TryFrom;

struct Console {
    acc: i32,
    pc: i32,
    instr: Vec<(String, i32)>,
}

impl Console {
    fn run_until_loop(&mut self) {
        let mut seen = HashMap::new();
        loop {
            seen.insert(self.pc.clone(), 1);
            let (op, val) = self.get_instr();
            if op.as_str() == "jmp" {
                let next_pc = self.pc + val;
                if seen.contains_key(&next_pc) {
                    break;
                }
            }
            self.tick();
            if self.terminated() {
                break;
            }
        }
    }

    fn terminated(&self) -> bool {
        self.pc >= self.instr.len() as i32
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    fn get_instr(&mut self) -> (String, i32) {
        self.instr[usize::try_from(self.pc).unwrap()].clone()
    }

    fn set_instr(&mut self, index: usize, instr: (String, i32)) {
        self.instr[index] = instr;
    }

    fn tick(&mut self) {
        let (op, val) = self.get_instr();
        match op.as_str() {
            "acc" => {
                self.acc = self.acc + val;
                self.pc = self.pc + 1;
            },
            "jmp" => {
                self.pc = self.pc + val;
            },
            "nop" => {
                self.pc = self.pc + 1;
            },
            _ => {
                panic!("Encountered invalid opcode '{}' at {}", op, self.pc);
            }
        }
    }
}

pub struct Day8 {
    console: Console,
}

impl Day8 {
    pub fn new(input: &Vec<&str>) -> Day8 {
        let mut instr = vec![];
        for line in input {
            let parts: Vec<&str> = line.split(" ").collect();
            instr.push((
                parts[0].to_string(),
                parts[1].parse::<i32>().unwrap(),
            ));
        }
        Day8 {
            console: Console {
                acc: 0,
                pc: 0,
                instr: instr,
            }
        }
    }
}

impl Solution for Day8 {
    fn part_1(&mut self) -> String {
        let console = &mut self.console;
        console.run_until_loop();
        console.acc.to_string()
    }

    fn part_2(&mut self) -> String {
        let console = &mut self.console;
        let instr = console.instr.clone();
        for (index, (op, val)) in instr.iter().enumerate() {
            let replacement = match op.as_str() {
                "jmp" => {
                    Some(("nop".to_string(), *val))
                },
                "nop" => {
                    Some(("jmp".to_string(), *val))
                },
                _ => {
                    None
                }
            };
            if replacement.is_some() {
                console.reset();
                console.set_instr(index, replacement.unwrap());
                console.run_until_loop();
                if console.terminated() {
                    break;
                }
                else {
                    console.set_instr(index, (op.clone(), *val));
                }
            }
        }
        console.acc.to_string()
    }
}
