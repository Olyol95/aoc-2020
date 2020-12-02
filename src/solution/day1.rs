use crate::solution::Solution;

pub struct Day1 {}

impl Solution for Day1 {
    fn part_1(&self, input: &Vec<&str>) -> String {
        let mut input_mut = parse_input(input);
        input_mut.sort();

        for tortoise in 0..input_mut.len() - 1 {
            let x = input_mut[tortoise];
            for hare in tortoise + 1..input_mut.len() {
                let y = input_mut[hare];
                if x + y == 2020 {
                    return (x * y).to_string();
                } else if x + y > 2020 {
                    break;
                }
            }
        }

        "0".to_string()
    }

    fn part_2(&self, input: &Vec<&str>) -> String {
        let mut input_mut = parse_input(input);
        input_mut.sort();

        for tortoise in 0..input_mut.len() - 2 {
            let x = input_mut[tortoise];
            for hare in tortoise + 1..input_mut.len() - 1 {
                let y = input_mut[hare];
                if x + y > 2020 {
                    break;
                }
                for cheetah in hare + 1..input_mut.len() {
                    let z = input_mut[cheetah];
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

fn parse_input(input: &Vec<&str>) -> Vec<i32> {
    input
        .to_vec()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
