use crate::solution::Solution;

use std::collections::HashMap;
use std::convert::TryInto;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point {
            x: x,
            y: y,
            z: z,
        }
    }
}

pub struct Day17 {
    state: Vec<HashMap<(i32, i32, i32), bool>>,
}

impl Day17 {
    pub fn new(input: &Vec<&str>) -> Day17 {
        let mut cells: HashMap<(i32, i32, i32), bool> = HashMap::new();
        for y in 0..input.len() {
            let mut x = 0;
            for c in input[y].chars() {
                if c == '#' {
                    cells.insert((x, y.try_into().unwrap(), 0), true);
                }
                x = x + 1;
            }
        }
        Day17 {
            state: vec![cells, HashMap::new()],
        }
    }

    fn is_active(&mut self, point: &Point) -> bool {
        self.state[0].contains_key(&(point.x, point.y, point.z))
    }

    fn set_active(&mut self, point: &Point, active: bool) {
        if active {
            self.state[1].insert((point.x, point.y, point.z), active);
        }
        else {
            self.state[1].remove(&(point.x, point.y, point.z));
        }
    }

    fn get_active(&mut self) -> Vec<Point> {
        let mut active = vec![];
        for (x, y, z) in self.state[0].keys() {
            active.push(Point::new(*x, *y, *z));
        }
        active
    }

    fn bounds(&mut self) -> Vec<Point> {
        let mut bounds = vec![Point::new(0, 0, 0), Point::new(0, 0, 0)];
        for point in self.get_active() {
            if point.x < bounds[0].x {
                bounds[0].x = point.x;
            }
            if point.x > bounds[1].x {
                bounds[1].x = point.x;
            }
            if point.y < bounds[0].y {
                bounds[0].y = point.y;
            }
            if point.y > bounds[1].y {
                bounds[1].y = point.y;
            }
            if point.z < bounds[0].z {
                bounds[0].z = point.z;
            }
            if point.z > bounds[1].z {
                bounds[1].z = point.z;
            }
        }
        bounds
    }

    fn bounded_points(&mut self) -> Vec<Point> {
        let mut points = vec![];
        let bounds = self.bounds();
        for x in bounds[0].x - 1 .. bounds[1].x + 2 {
            for y in bounds[0].y - 1 .. bounds[1].y + 2 {
                for z in bounds[0].z - 1 .. bounds[1].z + 2 {
                    points.push(Point::new(x, y, z)); 
                }
            }
        }
        points
    }

    fn active_neighbours(&mut self, point: &Point) -> usize {
        let mut active = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    let neighbour = Point::new(
                        point.x + dx,
                        point.y + dy,
                        point.z + dz,
                    );
                    if self.is_active(&neighbour) {
                        active = active + 1;
                    }
                }
            }
        }
        active
    }

    fn debug(&mut self) {
        let bounds = self.bounds();
        for z in bounds[0].z - 1 .. bounds[1].z + 2 {
            println!("Z: {}", z);
            for y in bounds[0].y - 1 .. bounds[1].y + 2 {
                for x in bounds[0].x - 1 .. bounds[1].x + 2 {
                    if self.is_active(&Point::new(x, y, z)) {
                        print!("#");
                    }
                    else {
                        print!(".");
                    }
                }
                print!("\n");
            }
            print!("\n");
        }
        println!("-----");
    }

    fn tick(&mut self) {
        for point in &self.bounded_points() {
            let neighbours = self.active_neighbours(point);
            if self.is_active(point) {
                self.set_active(point, neighbours == 2 || neighbours == 3);
            }
            else {
                self.set_active(point, neighbours == 3);
            }
        }
        self.state[0] = self.state[1].clone();
        self.state[1].clear();
    }
}

impl Solution for Day17 {
    fn part_1(&mut self) -> String {
        for _cycles in 0..6 {
            self.tick();
        }
        self.get_active().len().to_string()
    }

    fn part_2(&mut self) -> String {
        "0".to_string()
    }
}
