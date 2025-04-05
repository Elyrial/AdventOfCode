use std::time::{Duration, Instant};

pub struct Day01;

impl super::super::Solution for Day01 {
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

impl Day01 {
    pub fn part1(&self, input: &str) -> String {
        let sum = input.chars().fold(0, |acc, c| {
            acc + match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        });
        sum.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut floor = 0;

        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {},
            }

            if floor == -1 {
                return (i + 1).to_string();
            }
        }

        "Not found".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "))(((((";
    const TEST_INPUT_P2: &str = "()())";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "5");
    }
}

