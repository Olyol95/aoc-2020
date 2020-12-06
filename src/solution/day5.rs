use crate::solution::Solution;

struct BoardingPass {
    bsp: Vec<bool>,
}

impl BoardingPass {
    fn row(&self) -> u8 {
        BoardingPass::partition((0, 127), &mut self.bsp[3..].to_vec())
    }

    fn column(&self) -> u8 {
        BoardingPass::partition((0, 7), &mut self.bsp[..3].to_vec())
    }

    fn id(&self) -> u16 {
        u16::from(self.row()) * 8 + u16::from(self.column())
    }

    fn partition(range: (u8, u8), bsp: &mut Vec<bool>) -> u8 {
        if bsp.is_empty() {
            return range.0;
        }
        let midpoint = range.0 + ((range.1 - range.0) / 2);
        match bsp.pop().unwrap() {
            true  => BoardingPass::partition((midpoint + 1, range.1), bsp),
            false => BoardingPass::partition((range.0, midpoint), bsp),
        }
    }
}

pub struct Day5 {
    boarding_passes: Vec<BoardingPass>,
}

impl Day5 {
    pub fn new(input: &Vec<&str>) -> Day5 {
        Day5 {
            boarding_passes: input.iter().map(|bsp| BoardingPass {
                bsp: bsp.chars().rev().map(|c| c == 'B' || c == 'R').collect(),
            }).collect(),
        }
    }
}

impl Solution for Day5 {
    fn part_1(&self) -> String {
        self.boarding_passes.iter()
            .map(|pass| pass.id())
            .max()
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut ids: Vec<u16> = self.boarding_passes.iter()
            .map(|p| p.id())
            .collect();
        ids.sort();
        for (idx, id) in ids.iter().enumerate() {
            if ids[idx + 1] != id + 1 {
                return (id + 1).to_string();
            }
        }
        "0".to_string()
    }
}
