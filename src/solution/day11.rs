use crate::solution::Solution;

use std::cmp;
use std::convert::TryInto;

enum Part {
    One,
    Two,
}

pub struct Day11 {
    seats: Vec<Vec<char>>,
}

impl Day11 {
    pub fn new(input: &Vec<&str>) -> Day11 {
        let mut seats = vec![];
        for line in input {
            seats.push(line.chars().collect());
        }
        Day11 {
            seats: seats,
        }
    }

    fn run(&mut self, part: Part) -> usize {
        let init_state = self.seats.clone();
        let mut occupied = self.count_occupied();
        loop {
            self.tick(&part);
            let next = self.count_occupied();
            if occupied == next {
                break;
            }
            occupied = next;
        }
        self.seats = init_state;
        occupied
    }

    fn tick(&mut self, part: &Part) {
        let mut next_state = vec![];
        for y in 0..self.seats.len() {
            let mut next_row = vec![];
            for x in 0..self.seats[y].len() {
                next_row.push(match self.seats[y][x] {
                    '.' => '.',
                    'L' => {
                        if match part {
                            Part::One => self.count_adjacent(x, y) == 0,
                            Part::Two => self.count_visible(x, y) == 0,
                        } { '#' } else { 'L' }
                    },
                    '#' => {
                        if match part {
                            Part::One => self.count_adjacent(x, y) >= 4,
                            Part::Two => self.count_visible(x, y) >= 5,
                        } { 'L' } else { '#' }
                    },
                    _ => {
                        panic!("Unexpected character found at {},{}", x, y);
                    },
                });
            }
            next_state.push(next_row);
        }
        self.seats = next_state;
    }

    fn count_adjacent(&mut self, x: usize, y: usize) -> usize {
        let mut total = 0;
        let dy_min = if y > 0 { y - 1 } else { 0 };
        let dx_min = if x > 0 { x - 1 } else { 0 };
        for dy in dy_min .. cmp::min(self.seats.len(), y + 2) {
            for dx in dx_min .. cmp::min(self.seats[y].len(), x + 2) {
                if self.seats[dy][dx] == '#' && !(dx == x && dy == y) {
                    total = total + 1;
                }
            }
        }
        total
    }

    fn count_visible(&mut self, x: usize, y: usize) -> usize {
        let mut total = 0;
        for dy in (-1 .. 2).collect::<Vec<i32>>() {
            for dx in (-1 .. 2).collect::<Vec<i32>>() {
                let mut y_ind = y;
                let mut x_ind = x;
                loop {
                    if dy == 0 && dx == 0 {
                        break;
                    }

                    if (dy < 0 && y_ind <= 0)
                        || (dy > 0 && y_ind >= self.seats.len() - 1) {
                        break;
                    }
                    else {
                        y_ind = (y_ind as i32 + dy).try_into().unwrap();
                    }

                    if (dx < 0 && x_ind <= 0)
                        || (dx > 0 && x_ind >= self.seats[y_ind].len() - 1) {
                        break;
                    }
                    else {
                        x_ind = (x_ind as i32 + dx).try_into().unwrap();
                    }

                    if x_ind == x && y_ind == y {
                        continue;
                    }

                    match self.seats[y_ind][x_ind] {
                        '#' => {
                            total = total + 1;
                            break;
                        },
                        'L' => {
                            break;
                        },
                        _ => {},
                    }
                }
            }
        }
        total
    }

    fn count_occupied(&mut self) -> usize {
        self.seats.iter()
            .map(|row| row.iter()
                 .filter(|c| **c == '#')
                 .count()
            )
            .fold(0, |a, b| a + b)
    }
}

impl Solution for Day11 {
    fn part_1(&mut self) -> String {
        self.run(Part::One).to_string()
    }

    fn part_2(&mut self) -> String {
        self.run(Part::Two).to_string()
    }
}
