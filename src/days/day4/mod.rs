use super::utils;
use super::Day;
use std::str::FromStr;

pub struct Interval2D(u32, u32);

impl Interval2D {
    fn does_overlap(&self, other: &Self) -> bool {
        other.0 <= self.1 && other.1 >= self.0
    }
}

pub struct Line(Interval2D, Interval2D);

impl Line {
    fn has_overlap(&self) -> bool {
        self.0.does_overlap(&self.1)
    }
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new("(\\d+)-(\\d+)").unwrap();
        let mut intervals = re.captures_iter(s).map(|cap| {
            let p1 = cap.get(1).and_then(|m| m.as_str().parse().ok()).unwrap();
            let p2 = cap.get(2).and_then(|m| m.as_str().parse().ok()).unwrap();
            Interval2D(p1, p2)
        });

        Ok(Line(intervals.next().unwrap(), intervals.next().unwrap()))
    }
}

pub struct Day4;

impl Day<u32> for Day4 {
    fn run() -> Result<u32, String> {
        Ok(utils::fetch_input(4)?
            .lines()
            .into_iter()
            .map(Line::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(Line::has_overlap)
            .map(|b| b as u32)
            .sum())
    }
}
