use std::time::Duration;

pub mod solutions;

/// Implements the [`Solution`] trait for a day struct, timing each part separately.
///
/// The struct must have:
/// - `part1(&self, input: &str) -> String`
/// - `part2(&self, input: &str) -> String`
#[macro_export]
macro_rules! impl_solution {
    ($day:ident) => {
        impl $crate::Solution for $day {
            fn solve(
                &self,
                input: &str,
            ) -> (String, String, ::std::time::Duration, ::std::time::Duration) {
                let start = ::std::time::Instant::now();
                let p1 = self.part1(input);
                let t1 = start.elapsed();

                let start = ::std::time::Instant::now();
                let p2 = self.part2(input);
                let t2 = start.elapsed();

                (p1, p2, t1, t2)
            }
        }
    };
}

pub trait Solution {
    fn solve(&self, input: &str) -> (String, String, Duration, Duration);
}

pub fn get_solution(year: u16, day: u8) -> anyhow::Result<Box<dyn Solution>> {
    solutions::get_solution(year, day)
}
