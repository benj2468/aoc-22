use super::{utils, Day};
use std::str::FromStr;

pub struct Line(u32, u32);

impl Line {
    fn score(&self) -> u32 {
        let score = if self.1 == ((self.0 + 1) % 3) {
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
        let re = regex::Regex::new(r"([ABC]) ([XYZ])").unwrap();
        let mut groups = re.captures_iter(s);
        let first = groups.next().unwrap();
        let op: i32 = match &first[1] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!(),
        };
        let me = match &first[2] {
            "X" => op - 1,
            "Y" => op,
            "Z" => op + 1,
            _ => panic!(),
        }
        .rem_euclid(3);

        Ok(Line(op as u32, me as u32))
    }
}
pub struct Day2 {}

impl Day<u32> for Day2 {
    fn run() -> Result<u32, String> {
        let formatted_path = format!("src/days/day2/{}.txt", utils::run_ty());
        let path = std::path::Path::new(&formatted_path);
        Ok(std::fs::read_to_string(path)
            .map_err(|e| e.to_string())?
            .lines()
            .into_iter()
            .map(Line::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(Line::score)
            .sum::<u32>())
    }
}
