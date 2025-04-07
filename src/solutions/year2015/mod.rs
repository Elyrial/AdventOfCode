use super::super::Solution;
use anyhow::{bail, Result};

pub mod day01;
pub mod day02;
pub mod day03;

pub fn get_solution(day: u8) -> Result<Box<dyn Solution>> {
    match day {
        1 => Ok(Box::new(day01::Day01)),
        2 => Ok(Box::new(day02::Day02)),
        3 => Ok(Box::new(day03::Day03)),
        _ => bail!("Day {} not implemented for year 2015", day),
    }
}

