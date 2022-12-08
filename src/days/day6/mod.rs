use itertools::Itertools;

use super::utils;
use super::Day;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Index;
use std::str::FromStr;

pub struct Day6;

impl Day<u32> for Day6 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(6)?;

        let mut window = vec![];

        for (i, c) in input.chars().enumerate() {
            while window.contains(&c) {
                window.remove(0);
            }
            window.push(c);
            if window.len() == 14 {
                return Ok(i as u32 + 1);
            }
        }

        Err("Invalid data...".to_string())
    }
}
