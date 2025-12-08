use super::super::Solution;
use anyhow::{bail, Result};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day07;
pub mod day08;

pub fn get_solution(day: u8) -> Result<Box<dyn Solution>> {
    match day {
        1 => Ok(Box::new(day01::Day01)),
        2 => Ok(Box::new(day02::Day02)),
        3 => Ok(Box::new(day03::Day03)),
        4 => Ok(Box::new(day04::Day04)),
        5 => Ok(Box::new(day05::Day05)),
        7 => Ok(Box::new(day07::Day07)),
        8 => Ok(Box::new(day08::Day08)),
        _ => bail!("Day {} not implemented for year 2025", day),
    }
}


