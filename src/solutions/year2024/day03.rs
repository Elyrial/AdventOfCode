use regex::Regex;
use std::time::{Duration, Instant};

pub struct Day03;

impl super::super::Solution for Day03 {
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

impl Day03 {
    pub fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)")
            .expect("Invalid regex input");

        let mut total_sum = 0;

        for cap in re.captures_iter(input) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            total_sum += x * y;
        }

        total_sum.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
            .expect("Invalid regex input");

        let mut total_sum = 0;
        let mut enabled = true;

        for cap in re.captures_iter(input) {
            if let Some(matched) = cap.get(0) {
                match matched.as_str() {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    mul_expr if mul_expr.starts_with("mul(") && enabled => {
                        if let (Ok(x), Ok(y)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                            total_sum += x * y;
                        }
                    },
                    _ => (),
                }
            }
        }

        total_sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_P2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(Day03.part1(TEST_INPUT_P1), "161");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03.part2(TEST_INPUT_P2), "48");
    }
}

