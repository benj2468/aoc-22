use std::collections::HashSet;

use super::utils::Point;
use super::{utils, Day};

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Day9 {
    knots: Vec<Point>,
    solution: HashSet<Point>,
}

impl Day9 {
    fn step(&mut self, dir: &Direction, idx: usize) {
        if idx == self.knots.len() - 1 {
            let mut curr_knot = self.knots.get_mut(idx).unwrap();
            match dir {
                Direction::Right => {
                    curr_knot.x += 1;
                }
                Direction::Left => {
                    curr_knot.x -= 1;
                }
                Direction::Up => {
                    curr_knot.y += 1;
                }
                Direction::Down => {
                    curr_knot.y -= 1;
                }
            }
            self.step(dir, idx - 1)
        } else {
            let prev_knot = *self.knots.get(idx + 1).unwrap();
            let mut curr_knot = self.knots.get_mut(idx).unwrap();

            // Realign the current knot with the previous knot
            if curr_knot.y == prev_knot.y {
                // Same row
                if curr_knot.x < prev_knot.x - 1 {
                    curr_knot.x = prev_knot.x - 1;
                }
                if curr_knot.x > prev_knot.x + 1 {
                    curr_knot.x = prev_knot.x + 1
                }
            } else if curr_knot.x == prev_knot.x {
                // Same column
                if curr_knot.y < prev_knot.y - 1 {
                    curr_knot.y = prev_knot.y - 1;
                }
                if curr_knot.y > prev_knot.y + 1 {
                    curr_knot.y = prev_knot.y + 1
                }
            } else {
                let dist = prev_knot.manhattan_distance(curr_knot);
                if dist > 2 {
                    // Different row and different column
                    if prev_knot.x > curr_knot.x {
                        curr_knot.x += 1;
                    } else {
                        curr_knot.x -= 1;
                    }
                    if prev_knot.y > curr_knot.y {
                        curr_knot.y += 1;
                    } else {
                        curr_knot.y -= 1;
                    }
                }
            }

            if idx == 0 {
                self.solution.insert(*curr_knot);
            } else {
                self.step(dir, idx - 1)
            }
        }
    }
    fn digest(&mut self, dir: Direction, count: isize) {
        for _ in 0..count {
            self.step(&dir, self.knots.len() - 1);
        }
        for p in &self.knots {
            println!("{:?}", p);
        }
    }
}

impl Day<u32> for Day9 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(9)?;

        let mut day = Self {
            knots: vec![Point::default(); 10],
            ..Default::default()
        };

        input.lines().into_iter().for_each(|line| {
            let mut line = line.split(' ');
            let dir = Direction::from(line.next().unwrap());
            let count = line.next().unwrap().parse::<isize>().unwrap();
            println!("{:?} - {:?}", dir, count);

            day.digest(dir, count)
        });

        Ok(day.solution.len() as u32)
    }
}
