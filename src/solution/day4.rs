use crate::solution::Solution;

extern crate regex;
use regex::Regex;

use std::collections::HashMap;

pub struct Day4 {}

#[derive(Clone)]
struct Field {
    name: String,
    value: String,
}

impl Field {
    fn new(name: &str, value: &str) -> Field {
        Field {
            name: String::from(name),
            value: String::from(value),
        }
    }

    fn is_valid(&self) -> bool {
        match self.name.as_str() {
            "byr" => {
                let pattern = Regex::new(r"^\d{4}$").unwrap();
                let intval = self.value.parse::<i32>().unwrap_or(0);
                pattern.is_match(&self.value)
                    && intval >= 1920
                    && intval <= 2002
            },
            "iyr" => {
                let pattern = Regex::new(r"^\d{4}$").unwrap();
                let intval = self.value.parse::<i32>().unwrap_or(0);
                pattern.is_match(&self.value)
                    && intval >= 2010
                    && intval <= 2020
            },
            "eyr" => {
                let pattern = Regex::new(r"^\d{4}$").unwrap();
                let intval = self.value.parse::<i32>().unwrap_or(0);
                pattern.is_match(&self.value)
                    && intval >= 2020
                    && intval <= 2030
            },
            "hgt" => {
                let pattern = Regex::new(r"^\d+(cm|in)$").unwrap();
                let mut int_sub = self.value.clone();
                int_sub.truncate(self.value.len() - 2);
                let intval = int_sub.parse::<i32>().unwrap_or(0);
                pattern.is_match(&self.value)
                    && match &pattern.captures_iter(&self.value).next().unwrap()[1] {
                        "cm" => {
                            intval >= 150
                                && intval <= 193
                        },
                        "in" => {
                            intval >= 59
                                && intval <= 76
                        },
                        _ => false,
                    }
            },
            "hcl" => {
                let pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                pattern.is_match(&self.value)
            },
            "ecl" => {
                let pattern = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                pattern.is_match(&self.value)
            },
            "pid" => {
                let pattern = Regex::new(r"^\d{9}$").unwrap();
                pattern.is_match(&self.value)
            },
            _ => false,
        }
    }
}

struct Passport {
    fields: HashMap<String, Field>,
}

impl Passport {
    fn required_fields(&self) -> Vec<&str> {
        vec![
            "byr", "iyr", "eyr", "hgt",
            "hcl", "ecl", "pid"
        ]
    }

    fn has_field(&self, field_name: &str) -> bool {
        self.fields.contains_key(field_name)
    }

    fn get_field(&self, field_name: &str) -> &Field {
        self.fields.get(field_name).unwrap()
    }

    fn has_required_fields(&self) -> bool {
        self.required_fields().iter()
            .map(|field| self.has_field(&field))
            .fold(true, |prev, valid| prev && valid)
    }

    fn is_valid(&self) -> bool {
        self.has_required_fields() &&
            self.required_fields().iter()
                .map(|field| self.get_field(field).is_valid())
                .fold(true, |prev, valid| prev && valid)
    }
}

impl Solution for Day4 {
    fn part_1(&self, input: &Vec<&str>) -> String {
        parse_input(input).iter()
            .filter(|p| p.has_required_fields())
            .count()
            .to_string()
    }

    fn part_2(&self, input: &Vec<&str>) -> String {
        parse_input(input).iter()
            .filter(|p| p.is_valid())
            .count()
            .to_string()
    }
}

fn parse_input(input: &Vec<&str>) -> Vec<Passport> {
    let mut passports = vec![];
    let mut fields = HashMap::new();
    for (idx, line) in input.iter().enumerate() {
        if !line.is_empty() {
            for item in line.split(" ") {
                let (key, value) = item.split_at(3);
                fields.insert(String::from(key), Field::new(key, &value[1..]));
            }
        }
        if line.is_empty() || idx == input.len() - 1 {
            passports.push(
                Passport {
                    fields: fields.clone(),
                }
            );
            fields.clear();
        }
    }
    passports
}
