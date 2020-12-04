use crate::solution::Solution;

use std::convert::TryInto;

pub struct Day3 {}

struct Topology {
    tiles: Vec<Vec<bool>>,
}

impl Topology {
    fn height(&self) -> u16 {
        self.tiles.len().try_into().unwrap()
    }

    fn width(&self) -> u16 {
        self.tiles[0].len().try_into().unwrap()
    }

    fn is_passable(&self, x: u16, y: u16) -> bool {
        let idx_x = usize::from(x % self.width());
        let idx_y = usize::from(y % self.height());
        self.tiles[idx_y][idx_x]
    }
}

impl Solution for Day3 {
    fn part_1(&self, input: &Vec<&str>) -> String {
        let landscape = parse_input(input);
        count_impassable(&landscape, &3, &1).to_string()
    }

    fn part_2(&self, input: &Vec<&str>) -> String {
        let landscape = parse_input(input);
        let mut totals = vec![];
        let (mut inc_x, mut inc_y) = (1, 1);
        for _ in 0..5 {
            totals.push(count_impassable(&landscape, &inc_x, &inc_y));
            if inc_x < 7 {
                inc_x = inc_x + 2;
            }
            else {
                inc_x = 1;
                inc_y = 2;
            }
        }
        totals.iter().fold(1u32, |a, b| a * b).to_string()
    }
}

fn count_impassable(landscape: &Topology, inc_x: &u16, inc_y: &u16) -> u32 {
    let mut total_trees = 0;
    let (mut x, mut y) = (0, 0);
    while y < landscape.height() {
        if !landscape.is_passable(x, y) {
            total_trees = total_trees + 1;
        }
        x = x + inc_x;
        y = y + inc_y;
    }
    total_trees
}

fn parse_input(input: &Vec<&str>) -> Topology {
    let mut tiles = vec![];
    for line in input {
        tiles.push(
            line.chars().map(|c| c == '.').collect()
        );
    }
    Topology {
        tiles: tiles,
    }
}
