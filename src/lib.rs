use std::time::Duration;

pub mod solutions;

pub trait Solution {
    fn solve(&self, input: &str) -> (String, String, Duration, Duration);
}

pub fn get_solution(year: u16, day: u8) -> anyhow::Result<Box<dyn Solution>> {
    match year {
        2015 => solutions::year2015::get_solution(day),
        2016 => solutions::year2016::get_solution(day),
        2017 => solutions::year2017::get_solution(day),
        2018 => solutions::year2018::get_solution(day),
        2019 => solutions::year2019::get_solution(day),
        2020 => solutions::year2020::get_solution(day),
        2021 => solutions::year2021::get_solution(day),
        2023 => solutions::year2023::get_solution(day),
        _ => anyhow::bail!("Year not implemented"),
    }
}
