use std::collections::{HashMap, HashSet};

use regex::Captures;

use super::{utils::fetch_input, Day};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn man_dist(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<Captures<'_>> for Point {
    fn from(cap: Captures) -> Self {
        Self {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
        }
    }
}

pub struct Day15;

impl Day<u32> for Day15 {
    fn run() -> Result<u32, String> {
        let input = fetch_input(15)?;

        let mut beacons = HashSet::new();

        let max = 4000000_i32;

        let data: HashMap<_, _> = input
            .lines()
            .map(|l| {
                let re = regex::Regex::new("x=(-?\\d+), y=(-?\\d+)").unwrap();

                let mut groups = re.captures_iter(l);
                let sensor: Point = groups.next().unwrap().into();
                let beacon: Point = groups.next().unwrap().into();

                beacons.insert(beacon);

                let dist = sensor.man_dist(&beacon) as i32;

                (sensor, dist as u32)
            })
            .collect();

        // let min_x = data.iter().map(|(b, d)| b.x - *d as i32).min().unwrap();

        // let max_x = data.iter().map(|(b, d)| b.x + *d as i32).max().unwrap();

        // let mut res = 0;

        // println!("{:?} - {:?}", min_x, max_x);

        // for x in min_x..max_x {
        //     let p = Point { x, y: 2000000 }; // 2000000
        //     if beacons.contains(&p) {
        //         continue;
        //     }
        //     if data.iter().any(|(s, d)| p.man_dist(s) <= *d) {
        //         res += 1;
        //         continue;
        //     }
        // }

        let mut x = 0;
        while x <= max {
            let mut y = 0;
            while y <= max {
                let p = Point { x, y };
                let (s, d) = match data.iter().find(|(s, d)| s.man_dist(&p) <= **d) {
                    Some(x) => x,
                    None => {
                        println!("{:?}", (p.x as u128 * max as u128) + p.y as u128);
                        return Ok(1);
                    }
                };
                let rem_dist = d - s.man_dist(&p);
                if rem_dist == 0 {
                    y += 1;
                }
                y += rem_dist as i32;
            }
            x += 1;
        }
        Ok(1)
    }
}
