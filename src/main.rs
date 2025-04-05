use advent_of_code::{get_solution, Solution};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        year: u16,
        day: u8,
        #[arg(short, long)]
        input: Option<PathBuf>,
    },
    Fetch {
        year: u16,
        day: u8,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { year, day, input } => {
            let input = match input {
                Some(path) => std::fs::read_to_string(path)?,
                None => std::fs::read_to_string(format!("inputs/year{}/day{:02}.txt", year, day))?,
            };

            let solution = get_solution(year, day)?;
            let (p1, p2, t1, t2) = solution.solve(&input);

            println!("Part 1 ({}µs): {}", t1.as_micros(), p1);
            println!("Part 2 ({}µs): {}", t2.as_micros(), p2);
        }
        Commands::Fetch { year, day } => {
            println!("Run this to fetch cargo run --bin fetch_input {} {}", year, day);
        }
    }

    Ok(())
}
