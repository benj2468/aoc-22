use std::{collections::HashSet, str::FromStr};

use super::{utils, Day};

#[derive(Debug)]
pub struct Day14 {
    board: HashSet<Point>,
    floor: u32,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");

        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();

        Ok(Self { x, y })
    }
}

impl Point {
    fn line_to(&self, other: &Self) -> Vec<Point> {
        let mut res = vec![];
        if self.x == other.x {
            let min = std::cmp::min(other.y, self.y);
            let max = std::cmp::max(other.y, self.y);
            for y in min..max {
                res.push(Point { x: self.x, y })
            }
        }
        if self.y == other.y {
            let min = std::cmp::min(other.x, self.x);
            let max = std::cmp::max(other.x, self.x);
            for x in min..max {
                res.push(Point { x, y: self.y })
            }
        }

        res
    }

    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn down_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    fn down_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
}

impl FromStr for Day14 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board: HashSet<Point> = s
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|p| Point::from_str(p).unwrap())
                    .fold(vec![], |acc: Vec<Point>, item| {
                        let mut acc = acc;
                        match acc.last() {
                            Some(last) => {
                                let line = last.line_to(&item);
                                acc.extend(line);
                                acc.push(item);
                                acc
                            }
                            None => {
                                acc.push(item);
                                acc
                            }
                        }
                    })
            })
            .collect();

        let floor = board.iter().max_by_key(|p| p.y).unwrap().y + 2;

        Ok(Self { board, floor })
    }
}

impl Day14 {
    fn sand(&mut self) -> Option<Point> {
        let mut sand = Point { x: 500, y: 0 };

        while sand.y < self.floor - 1 {
            if !self.board.contains(&sand.down()) {
                sand = sand.down();
            } else if !self.board.contains(&sand.down_left()) {
                sand = sand.down_left();
            } else if !self.board.contains(&sand.down_right()) {
                sand = sand.down_right();
            } else {
                return Some(sand);
            }
        }
        Some(sand)
    }
}

impl Day<u32> for Day14 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(14)?;

        let mut day = Day14::from_str(input.as_str())?;

        let mut i = 0;
        while let Some(sand) = day.sand() && sand != (Point {x: 500, y: 0}) {
            day.board.insert(sand);
            i += 1
        }

        println!("{:?}", i + 1);

        todo!()
    }
}
