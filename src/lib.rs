use std::time::Duration;

pub mod solutions;

pub trait Solution {
    fn solve(&self, input: &str) -> (String, String, Duration, Duration);
}

pub fn get_solution(year: u16, day: u8) -> anyhow::Result<Box<dyn Solution>> {
    match year {
        2023 => solutions::year2023::get_solution(day),
        _ => anyhow::bail!("Year not implemented"),
    }
}
