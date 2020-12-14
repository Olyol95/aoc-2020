use crate::solution::Solution;

#[derive(Clone)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }

    fn rotate(&mut self, deg: i32) {
        let rads = (deg as f32).to_radians();
        let x0 = (self.x * (rads.cos() as i32)) - (self.y * (rads.sin() as i32));
        let y0 = (self.y * (rads.cos() as i32)) + (self.x * (rads.sin() as i32));
        self.x = x0;
        self.y = y0;
    }
}

#[derive(Clone)]
struct Ship {
    x: i32,
    y: i32,
    bearing: f32,
}

impl Ship {
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }

    fn rotate(&mut self, deg: i32) {
        let mut bearing = (self.bearing + deg as f32) % 360.0;
        if bearing < 0.0 {
            bearing = bearing + 360.0;
        }
        self.bearing = bearing;
    }

    fn impulse(&mut self, mag: i32) {
        let rads = self.bearing.to_radians();
        let dx = mag * (rads.sin() as i32);
        let dy = mag * (rads.cos() as i32);
        self.translate(dx, dy);
    }

    fn distance_from(&mut self, x0: i32, y0: i32) -> i32 {
        (self.x - x0).abs() + (self.y - y0).abs()
    }
}

pub struct Day12 {
    ship: Ship,
    waypoint: Waypoint,
    directions: Vec<(char, i32)>,
}

impl Day12 {
    pub fn new(input: &Vec<&str>) -> Day12 {
        let mut directions = vec![];
        for line in input {
            directions.push((
                line.chars().nth(0).unwrap(),
                line[1..].parse::<i32>().unwrap(),
            ));
        }
        Day12 {
            ship: Ship {
                x: 0,
                y: 0,
                bearing: 90.0,
            },
            waypoint: Waypoint {
                x: 10,
                y: 1,
            },
            directions: directions,
        }
    }
}

impl Solution for Day12 {
    fn part_1(&mut self) -> String {
        let init_ship = self.ship.clone();
        for (action, value) in &self.directions {
            match action {
                'N' => self.ship.translate(0, *value),
                'S' => self.ship.translate(0, -*value),
                'E' => self.ship.translate(*value, 0),
                'W' => self.ship.translate(-*value, 0),
                'L' => self.ship.rotate(-*value),
                'R' => self.ship.rotate(*value),
                'F' => self.ship.impulse(*value),
                _ => {
                    panic!("Encountered an invalid action type: {}", action);
                }
            }
        }
        let distance = self.ship.distance_from(0, 0);
        self.ship = init_ship;
        distance.to_string()
    }

    fn part_2(&mut self) -> String {
        let init_ship = self.ship.clone();
        let init_wayp = self.waypoint.clone();
        for (action, value) in &self.directions {
            match action {
                'N' => self.waypoint.translate(0, *value),
                'S' => self.waypoint.translate(0, -*value),
                'E' => self.waypoint.translate(*value, 0),
                'W' => self.waypoint.translate(-*value, 0),
                'L' => self.waypoint.rotate(*value),
                'R' => self.waypoint.rotate(360 - *value),
                'F' => {
                    let dx = self.waypoint.x * *value;
                    let dy = self.waypoint.y * *value;
                    self.ship.translate(dx, dy);
                },
                _ => {
                    panic!("Encountered an invalid action type: {}", action);
                }
            }
        }
        let distance = self.ship.distance_from(0, 0);
        self.ship = init_ship;
        self.waypoint = init_wayp;
        distance.to_string()
    }
}
