use super::{utils, Day};
pub struct Day1 {}

impl Day<u32> for Day1 {
    fn run() -> Result<u32, String> {
        let formatted_path = format!("src/days/day1/{}.txt", utils::run_ty());
        let path = std::path::Path::new(&formatted_path);
        std::fs::read_to_string(path)
            .map_err(|e| e.to_string())?
            .split("\n\n")
            .map(|elf| {
                String::from(elf)
                    .split("\n")
                    .filter(|e| !e.is_empty())
                    .map(|line| line.parse::<u32>())
                    .collect::<Result<Vec<u32>, _>>()
                    .map(|elf| elf.iter().sum())
            })
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| e.to_string())
            .map(|mut elves| {
                elves.sort();
                elves.iter().rev().take(3).sum::<u32>()
            })
        // .map(|elves| *elves.iter().max().expect("Oops there were no elves :("))
    }
}
