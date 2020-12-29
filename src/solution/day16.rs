use crate::solution::Solution;

use regex::Regex;

struct Rule {
    field: String,
    ranges: Vec<(u16, u16)>,
}

impl Rule {
    fn matches(&self, value: u16) -> bool {
        for (lbound, ubound) in &self.ranges {
            if *lbound <= value && value <= *ubound {
                return true;
            }
        }
        false
    }
}

pub struct Day16 {
    rules: Vec<Rule>,
    your_ticket: Vec<u16>,
    nearby_tickets: Vec<Vec<u16>>,
}

impl Day16 {
    pub fn new(input: &Vec<&str>) -> Day16 {
        let iter = input.split(|line| *line == "");

        let mut rules = vec![];
        let mut your_ticket = vec![];
        let mut nearby_tickets = vec![];

        for (idx, section) in iter.enumerate() {
            match idx {
                0 => {
                    let rule_regex = Regex::new(r"^([^:]+): (\d+)\-(\d+) or (\d+)\-(\d+)").unwrap();
                    for line in section {
                        let captures = rule_regex.captures(&line).unwrap();
                        rules.push(Rule {
                            field: captures[1].to_string(),
                            ranges: vec![
                                (
                                    captures[2].parse::<u16>().unwrap(),
                                    captures[3].parse::<u16>().unwrap()
                                ), (
                                    captures[4].parse::<u16>().unwrap(),
                                    captures[5].parse::<u16>().unwrap()
                                )
                            ],
                        });
                    }
                },
                1 => {
                    your_ticket = section[1].to_string().split(",")
                        .map(|s| s.parse::<u16>().unwrap()).collect();
                },
                2 => {
                    for line in section {
                        if *line == "nearby tickets:" {
                            continue;
                        }
                        nearby_tickets.push(
                            line.split(",").map(|s| s.parse::<u16>().unwrap()).collect()
                        );
                    }
                },
                _ => panic!("Unexpected new line found in input"),
            }
        }

        Day16 {
            rules: rules,
            your_ticket: your_ticket,
            nearby_tickets: nearby_tickets,
        }
    }
}

impl Solution for Day16 {
    fn part_1(&mut self) -> String {
        let mut invalid_fields = vec![];
        for ticket in &self.nearby_tickets {
            'value: for value in ticket {
                for rule in &self.rules {
                    if rule.matches(*value) {
                        continue 'value;
                    }
                }
                invalid_fields.push(*value);
            }
        }
        invalid_fields.iter().fold(0, |a, b| a + b).to_string()
    }

    fn part_2(&mut self) -> String {
        "0".to_string()
    }
}
