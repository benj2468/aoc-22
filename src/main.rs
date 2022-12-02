use days::Day;

mod days;

fn main() -> Result<(), String> {
    let mut args = std::env::args();

    let res = match args
        .nth(1)
        .ok_or("Not enough args provided".to_string())?
        .parse::<u32>()
        .map_err(|e| e.to_string())?
    {
        1 => days::day1::Day1::run()?,
        _ => panic!("Oh we haven't gotten there yet..."),
    };

    println!("Your Solution for today is:\n{}", res);

    Ok(())
}
