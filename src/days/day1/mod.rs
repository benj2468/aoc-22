use super::{utils, Day};
pub struct Day1 {}

impl Day<u32> for Day1 {
    fn run() -> Result<u32, String> {
        utils::fetch_input(1)?
            .split("\n\n")
            .map(|elf| {
                String::from(elf)
                    .split('\n')
                    .map(|line| line.parse::<u32>())
                    .collect::<Result<Vec<_>, _>>()
                    .map(|elf| elf.iter().sum())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
            .map(|mut elves| {
                elves.sort();
                elves.iter().rev().take(3).sum::<u32>()
            })
        // .map(|elves| *elves.iter().max().expect("Oops there were no elves :("))
    }
}
