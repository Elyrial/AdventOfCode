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
        let mut sum = 0;
        for (c1, c2) in input.chars().zip(input.chars().skip(1)) {
            if c1 == c2 {
                sum += c1.to_digit(10).unwrap();
            }
        }
        if let (Some(first), Some(last)) = (input.chars().next(), input.chars().last()) {
            if first == last {
                sum += first.to_digit(10).unwrap();
            }
        }

        sum.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let ans: u32 = input.chars()
            .take(input.len() / 2)
            .zip(input.chars().skip(input.len() / 2))
            .filter(|(a, b)| a == b)
            .map(|(c, _)| 2 * c.to_digit(10).unwrap())
            .sum();

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "91212129";
    const TEST_INPUT_P2: &str = "12131415";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "9");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "4");
    }
}

