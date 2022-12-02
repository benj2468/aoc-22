use std::fmt::Display;

pub trait Day<T: Display> {
    fn run() -> Result<T, String>;
}

pub mod day1;
pub mod day2;
mod utils;
