use crate::solution::Solution;

use std::collections::HashMap;

#[derive(Clone)]
struct Bag {
    colour: String,
    contains: Vec<(u32, Bag)>,
}

impl Bag {
    fn can_contain(&self, colour: &str) -> bool {
        for (_, bag) in &self.contains {
            if bag.colour == colour || bag.can_contain(colour) {
                return true;
            }
        }
        false
    }

    fn total_bags(&self) -> u32 {
        let mut total = 0;
        for (count, bag) in &self.contains {
            total = total + count + ( count * bag.total_bags() );
        }
        total
    }
}

pub struct Day7 {
    bags: HashMap<String, Bag>,
}

impl Day7 {
    pub fn new(input: &Vec<&str>) -> Day7 {
        let mut rules = HashMap::new();
        for line in input {
            let parts: Vec<&str> = line.split(" bags contain ").collect();
            let mut contains = vec![];
            for contain_str in parts[1].split(", ") {
                if contain_str.contains("no other bags") {
                    continue;
                }
                let rule: Vec<&str> = contain_str.split(" bag")
                    .next()
                    .unwrap()
                    .split(" ")
                    .collect();
                contains.push((
                    rule[0].parse::<u32>().unwrap(),
                    rule[1..].join(" "),
                ));
            }
            rules.insert(parts[0], contains);
        }

        let mut bags: HashMap<String, Bag> = HashMap::new();
        loop {
            let mut to_remove = vec![];
            'colour: for colour in rules.keys() {
                let mut contains = vec![];
                for (count, mapped_colour) in rules.get(colour).unwrap() {
                    if !bags.contains_key(mapped_colour) {
                        continue 'colour;
                    }
                    let bag = bags.get(mapped_colour).unwrap();
                    contains.push((*count, bag.clone()));
                }
                bags.insert(colour.to_string(), Bag {
                    colour: colour.to_string(),
                    contains: contains,
                });
                to_remove.push(colour.clone());
            }
            for colour in to_remove {
                rules.remove(colour);
            }
            if rules.keys().len() == 0 {
                break;
            }
        }

        Day7 {
            bags: bags,
        }
    }
}

impl Solution for Day7 {
    fn part_1(&self) -> String {
        self.bags.values()
            .filter(|bag| bag.can_contain("shiny gold"))
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.bags.get("shiny gold").unwrap()
            .total_bags()
            .to_string()
    }
}
