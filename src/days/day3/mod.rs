use itertools::Itertools;

use super::{utils, Day};
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

struct Sack {
    p1: HashSet<char>,
    p2: HashSet<char>,
}

impl Sack {
    fn shared(&self) -> u32 {
        self.p1
            .intersection(&self.p2)
            .into_iter()
            .map(|c| (*c as u32) - if c.is_uppercase() { 38 } else { 96 })
            .sum()
    }
}

impl FromStr for Sack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let (a, b) = s.split_at(len / 2);

        let p1 = a.chars().into_iter().collect::<HashSet<_>>();
        let p2 = b.chars().into_iter().collect::<HashSet<_>>();

        Ok(Self { p1, p2 })
    }
}

struct Group(Vec<HashSet<char>>);

pub struct Day3 {}

impl Day<u32> for Day3 {
    fn run() -> Result<u32, String> {
        let file_str = utils::fetch_input(3)?;
        let lines = file_str.lines().collect::<Vec<_>>();

        let mut score: u32 = 0;

        let mut i = 0;
        while i < lines.len() {
            let g0 = lines
                .get(i)
                .unwrap()
                .chars()
                .into_iter()
                .collect::<HashSet<_>>();
            let g1 = lines
                .get(i + 1)
                .unwrap()
                .chars()
                .into_iter()
                .collect::<HashSet<_>>();
            let g2 = lines
                .get(i + 2)
                .unwrap()
                .chars()
                .into_iter()
                .collect::<HashSet<_>>();

            let temp = g0
                .intersection(&g1)
                .into_iter()
                .cloned()
                .collect::<HashSet<char>>();
            score += temp
                .intersection(&g2)
                .into_iter()
                .map(|c| (*c as u32) - if c.is_uppercase() { 38 } else { 96 })
                .sum::<u32>();

            i += 3;
        }

        Ok(score)
    }
}
