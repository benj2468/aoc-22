use super::{utils, Day};
use std::str::FromStr;

pub struct Line(u32, u32);

impl Line {
    fn score(&self) -> u32 {
        let score = if self.1 == (self.0 + 1).rem_euclid(3) {
            6
        } else if self.1 == self.0 {
            3
        } else {
            0
        };
        self.1 + score + 1
    }
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"([ABC]) ([XYZ])").map_err(|e| e.to_string())?;
        let mut groups = re.captures_iter(s);
        let first = groups.next().ok_or("Bad Data")?;
        let op: i32 = match &first[1] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => return Err("Bad Data".to_string()),
        };
        let me = match &first[2] {
            "X" => op - 1,
            "Y" => op,
            "Z" => op + 1,
            _ => return Err("Bad Data".to_string()),
        }
        .rem_euclid(3);

        Ok(Line(op as u32, me as u32))
    }
}
pub struct Day2 {}

impl Day<u32> for Day2 {
    fn run() -> Result<u32, String> {
        Ok(utils::fetch_input(2)?
            .lines()
            .into_iter()
            .map(Line::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(Line::score)
            .sum::<u32>())
    }
}
