use itertools::Itertools;

use super::{utils, Day};

#[derive(Default)]
struct CRT(Vec<String>);

impl CRT {
    fn read(&mut self, cycle: usize, position: i32) {
        let pos = (cycle - 1).rem_euclid(40) as i32;
        if pos <= position + 1 && pos >= position - 1 {
            self.0.push("#".to_string());
        } else {
            self.0.push(".".to_string());
        }
    }
}

impl ToString for CRT {
    fn to_string(&self) -> String {
        self.0
            .chunks(40)
            .into_iter()
            .map(|row| row.join(""))
            .collect_vec()
            .join("\n")
    }
}

pub struct Day10 {}
impl Day<u32> for Day10 {
    fn run() -> Result<u32, String> {
        let input = utils::fetch_input(10)?;

        let mut cycle: i32 = 1;
        let mut value: i32 = 1;

        let mut crt = CRT::default();

        crt.read(1, 1);

        input.lines().for_each(|l| {
            let mut addition = 0;
            if l.starts_with("addx") {
                addition = l.split_at(5).1.parse::<i32>().unwrap();
                cycle += 1;
                crt.read(cycle as usize, value)
            }
            value += addition;
            cycle += 1;
            crt.read(cycle as usize, value)
        });

        println!("{}", crt.to_string());

        Ok(1)
    }
}
