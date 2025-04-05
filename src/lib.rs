use std::time::Duration;

pub mod solutions;

pub trait Solution {
    fn solve(&self, input: &str) -> (String, String, Duration, Duration);
}

pub fn get_solution(year: u16, day: u8) -> anyhow::Result<Box<dyn Solution>> {
    match year {
        2015 => solutions::year2015::get_solution(day),
        2016 => solutions::year2016::get_solution(day),
        2023 => solutions::year2023::get_solution(day),
        _ => anyhow::bail!("Year not implemented"),
    }
}
