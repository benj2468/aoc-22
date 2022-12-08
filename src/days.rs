use std::fmt::Display;

pub trait Day<T: Display> {
    fn run() -> Result<T, String>;
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
mod utils;
