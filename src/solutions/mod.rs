use crate::Solution;
use anyhow::{bail, Result};

// Include all years
pub mod year2015;
pub mod year2016;
pub mod year2023;

pub fn get_solution(year: u16, day: u8) -> Result<Box<dyn Solution>> {
    match year {
        2015 => year2015::get_solution(day),
        2016 => year2016::get_solution(day),
        2023 => year2023::get_solution(day),
        _ => bail!("Year {} not implemented", year),
    }
}
