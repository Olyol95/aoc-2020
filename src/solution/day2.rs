use crate::solution::Solution;

struct Policy {
    range_start: u8,
    range_end: u8,
    character: char,
}

impl Policy {
    fn is_fulfilled_old(&self, password: &String) -> bool {
        let mut total = 0;
        for c in password.chars() {
            if c == self.character {
                total = total + 1;
            }
        }
        total >= self.range_start
            && total <= self.range_end
    }

    fn is_fulfilled_new(&self, password: &String) -> bool {
        let pos1: usize = (self.range_start - 1).into();
        let pos2: usize = (self.range_end - 1).into();
        (password.chars().nth(pos1).unwrap() == self.character)
            != (password.chars().nth(pos2).unwrap() == self.character)
    }
}

struct Password {
    text: String,
    policy: Policy,
}

impl Password {
    fn new(text: &str, policy: Policy) -> Password {
        Password {
            text: text.to_string(),
            policy: policy,
        }
    }

    fn is_valid(&self, policy_name: &str) -> bool {
        match policy_name {
            "old" => self.policy.is_fulfilled_old(&self.text),
            "new" => self.policy.is_fulfilled_new(&self.text),
            _     => false,
        }
    }
}

pub struct Day2 {
    passwords: Vec<Password>,
}

impl Day2 {
    pub fn new(input: &Vec<&str>) -> Day2 {
        let mut passwords = vec![];
        for line in input {
            let parts: Vec<&str> = line.split(" ").collect();
            let range: Vec<&str> = parts[0].split("-").collect();
            passwords.push(
                Password::new(
                    parts[2],
                    Policy {
                        range_start: range[0].parse::<u8>().unwrap(),
                        range_end: range[1].parse::<u8>().unwrap(),
                        character: parts[1].chars().nth(0).unwrap(),
                    },
                )
            );
        }
        Day2 {
            passwords: passwords,
        }
    }
}

impl Solution for Day2 {
    fn part_1(&mut self) -> String {
        let mut total_valid = 0;
        for password in &self.passwords {
            if password.is_valid("old") {
                total_valid = total_valid + 1;
            }
        }
        total_valid.to_string()
    }

    fn part_2(&mut self) -> String {
        let mut total_valid = 0;
        for password in &self.passwords {
            if password.is_valid("new") {
                total_valid = total_valid + 1;
            }
        }
        total_valid.to_string()
    }
}
