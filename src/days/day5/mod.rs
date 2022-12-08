use super::utils;
use super::Day;
use std::str::FromStr;

#[derive(Debug)]
pub struct Line {
    from: u32,
    to: u32,
    count: u32,
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().filter_map(|w| w.parse::<u32>().ok());

        let count = split.next().unwrap();
        let from = split.next().unwrap() - 1;
        let to = split.next().unwrap() - 1;

        Ok(Self { from, to, count })
    }
}

pub struct State(Vec<Vec<String>>);

impl State {
    fn new() -> Self {
        let mut inner = vec![];
        for i in 0..10 {
            let mut inner_inner = vec![];
            for j in 0..10 {
                inner_inner.push(format!("{}:{}", i, j - 10))
            }
            inner.push(inner_inner);
        }
        Self(inner)
    }

    fn apply(&mut self, m: &Line) {
        for _ in 0..m.count {
            let b = self.0.get_mut(m.from as usize).unwrap().pop().unwrap();
            self.0.get_mut(m.to as usize).unwrap().push(b);
        }
    }
    fn apply2(&mut self, m: &Line) {
        let mut tmp = vec![];
        for _ in 0..m.count {
            let b = self.0.get_mut(m.from as usize).unwrap().pop().unwrap();
            tmp.push(b);
        }

        for _ in 0..m.count {
            let t = tmp.pop().unwrap();
            self.0.get_mut(m.to as usize).unwrap().push(t);
        }
    }
}

pub struct Day5;

impl Day<u32> for Day5 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(5)?;

        let mut state = State::new();

        input
            .split("\n\n")
            .nth(1)
            .unwrap()
            .lines()
            .map(Line::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .for_each(|m| state.apply2(m));

        state.0.iter().for_each(|s| println!("{:?}", s.last()));

        Ok(1)
        // TPGVQPFDH
        // DMRDFRHHH
    }
}
