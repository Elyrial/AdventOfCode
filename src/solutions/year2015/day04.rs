use md5;
use std::time::{Duration, Instant};

pub struct Day04;

impl super::super::Solution for Day04 {
    fn solve(&self, input: &str) -> (String, String, Duration, Duration) {
        let start = Instant::now();
        let p1 = self.part1(input);
        let t1 = start.elapsed();

        let start = Instant::now();
        let p2 = self.part2(input);
        let t2 = start.elapsed();

        (p1, p2, t1, t2)
    }
}

impl Day04 {
    pub fn part1(&self, input: &str) -> String {
        let mut num = 1;

        loop {
            let test_input = format!("{}{}", input, num);
            let hash = md5::compute(test_input.as_bytes());
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with("00000") {
                return num.to_string();
            }
            num += 1;
        }
    }

    pub fn part2(&self, input: &str) -> String {
        let mut num = 1;

        loop {
            let test_input = format!("{}{}", input, num);
            let hash = md5::compute(test_input.as_bytes());
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with("000000") {
                return num.to_string();
            }
            num += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "bgvyzdsv";
    const TEST_INPUT_P2: &str = "bgvyzdsv";

    #[test]
    fn test_part1() {
        assert_eq!(Day04.part1(TEST_INPUT_P1), "254575");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04.part2(TEST_INPUT_P2), "1038736");
    }
}

