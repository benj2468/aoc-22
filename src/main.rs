use days::Day;

mod days;

fn main() -> Result<(), String> {
    dotenv::dotenv().map_err(|e| e.to_string())?;

    let mut args = std::env::args();

    let res = match args
        .nth(1)
        .ok_or_else(|| "Not enough args provided".to_string())?
        .parse::<u32>()
        .map_err(|e| e.to_string())?
    {
        1 => days::day1::Day1::run()?,
        2 => days::day2::Day2::run()?,
        3 => days::day3::Day3::run()?,
        4 => days::day4::Day4::run()?,
        5 => days::day5::Day5::run()?,
        6 => days::day6::Day6::run()?,
        7 => days::day7::Day7::run()?,
        8 => days::day8::Day8::run()?,
        _ => panic!("Oh we haven't gotten there yet..."),
    };

    println!("Your Solution for today is:\n{}", res);

    Ok(())
}
