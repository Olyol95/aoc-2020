use crate::solution::Solution;

use std::collections::HashMap;

#[derive(Clone)]
struct Adapter {
    rating: u8,
}

impl Adapter {
    fn is_compatible(&self, jolts: u8) -> bool {
        self.rating <= jolts + 3
            && self.rating > jolts
    }
}

pub struct Day10 {
    adapters: Vec<Adapter>,
}

impl Day10 {
    pub fn new(input: &Vec<&str>) -> Day10 {
        let mut adapters: Vec<Adapter> = input.iter()
            .map(|s| Adapter {
                rating: s.parse::<u8>().unwrap(),
            })
            .collect();
        adapters.sort_by(|a, b| a.rating.cmp(&b.rating));
        Day10 {
            adapters: adapters,
        }
    }

    fn device_rating(&mut self) -> u8 {
        self.adapters.iter().map(|a| a.rating).max().unwrap() + 3
    }
}

impl Solution for Day10 {
    fn part_1(&mut self) -> String {
        let mut conn_rating = 0;
        let (mut d1, mut d3) = (0, 0);
        for adapter in &self.adapters {
            if adapter.is_compatible(conn_rating) {
                match adapter.rating - conn_rating {
                    1 => {
                        d1 = d1 + 1;
                    },
                    3 => {
                        d3 = d3 + 1;
                    }
                    _ => {},
                }
                conn_rating = adapter.rating;
            }
            else {
                panic!("Reached an unusable adapter for {} with rating {}!",
                       conn_rating, adapter.rating);
            }
        }
        (d1 * (d3 + 1)).to_string()
    }

    fn part_2(&mut self) -> String {
        let mut adapters = self.adapters.clone();
        adapters.reverse();
        adapters.push(Adapter { rating: 0 });

        let mut options: HashMap<u8, u64> = HashMap::new();
        options.insert(self.device_rating(), 1);

        for adapter in adapters {
            options.insert(
                adapter.rating,
                options.get(&(adapter.rating + 1)).unwrap_or(&0)
                    + options.get(&(adapter.rating + 2)).unwrap_or(&0)
                    + options.get(&(adapter.rating + 3)).unwrap_or(&0)
            );
        }

        options.get(&0).unwrap().to_string()
    }
}
