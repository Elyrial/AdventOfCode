use super::super::Solution;
use anyhow::{bail, Result};

pub mod day01;
// Add more days as needed

pub fn get_solution(day: u8) -> Result<Box<dyn Solution>> {
    match day {
        1 => Ok(Box::new(day01::Day01)),
        // Add more days as needed
        _ => bail!("Day {} not implemented for year 2022", day),
    }
}

