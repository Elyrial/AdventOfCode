use anyhow::{Context, Result};
use reqwest::header;
use std::{env, fs, path::PathBuf};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let session = env::var("AOC_SESSION")
        .context("Create .env file with AOC_SESSION=<cookie_value>")?;

    let args: Vec<String> = env::args().collect();
    let (year, day) = match args.as_slice() {
        [_, y, d] => (y.parse()?, d.parse()?),
        _ => anyhow::bail!("Usage: ./run.sh <year> <day>"),
    };

    let input = download_input(year, day, &session).await?;
    save_input(year, day, &input)?;

    println!("Saved input for {}/day{:02}", year, day);
    Ok(())
}

async fn download_input(year: u16, day: u8, session: &str) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::Client::new();

    client.get(&url)
        .header(header::COOKIE, format!("session={}", session))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
        .map_err(Into::into)
}

fn save_input(year: u16, day: u8, input: &str) -> Result<()> {
    let dir = format!("inputs/year{}", year);
    fs::create_dir_all(&dir)?;

    let path = PathBuf::from(dir).join(format!("day{:02}.txt", day));
    fs::write(path, input.trim())?;

    Ok(())
}
