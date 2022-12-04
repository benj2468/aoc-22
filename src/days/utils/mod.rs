use reqwest::header::COOKIE;

pub fn fetch_input(day: u32) -> Result<String, String> {
    match std::env::args()
        .nth(2)
        .unwrap_or_else(|| "-s".to_string())
        .as_str()
    {
        "-s" => fetch_sample(day),
        "-r" => fetch_real(day),
        _ => unimplemented!("Only support -s, or -r as a second argument"),
    }
}

fn fetch_sample(day: u32) -> Result<String, String> {
    let formatted_path = format!("src/days/day{}/sample.txt", day);
    let path = std::path::Path::new(&formatted_path);
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

fn fetch_real(day: u32) -> Result<String, String> {
    let token = std::env::var("AOC_TOKEN").map_err(|e| e.to_string())?;
    let formatted_token = format!("session={}", token);

    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let client = reqwest::blocking::Client::new();

    client
        .get(url)
        .header(COOKIE, formatted_token)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}
