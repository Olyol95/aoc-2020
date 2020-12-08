use crate::solution::Solution;

pub struct Day1 {
    expenses: Vec<i32>,
}

impl Day1 {
    pub fn new(input: &Vec<&str>) -> Day1 {
        let mut expenses = input.to_vec()
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        expenses.sort();
        Day1 {
            expenses: expenses,
        }
    }
}

impl Solution for Day1 {
    fn part_1(&mut self) -> String {
        for tortoise in 0..self.expenses.len() - 1 {
            let x = self.expenses[tortoise];
            for hare in tortoise + 1..self.expenses.len() {
                let y = self.expenses[hare];
                if x + y == 2020 {
                    return (x * y).to_string();
                } else if x + y > 2020 {
                    break;
                }
            }
        }

        "0".to_string()
    }

    fn part_2(&mut self) -> String {
        for tortoise in 0..self.expenses.len() - 2 {
            let x = self.expenses[tortoise];
            for hare in tortoise + 1..self.expenses.len() - 1 {
                let y = self.expenses[hare];
                if x + y > 2020 {
                    break;
                }
                for cheetah in hare + 1..self.expenses.len() {
                    let z = self.expenses[cheetah];
                    if x + y + z == 2020 {
                        return (x * y * z).to_string();
                    } else if x + y + z > 2020 {
                        break;
                    }
                }
            }
        }

        "0".to_string()
    }
}
