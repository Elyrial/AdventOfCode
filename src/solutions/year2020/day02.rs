use regex::Regex;
use std::time::{Duration, Instant};

pub struct Day02;

impl super::super::Solution for Day02 {
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

impl Day02 {
    pub fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")
            .expect("Invalid regex pattern");

        let mut ans = 0;

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let min = caps[1].parse::<usize>().unwrap();
                let max = caps[2].parse::<usize>().unwrap();
                let ch = caps[3].chars().next().unwrap();
                let s = &caps[4];

                let count = s.chars()
                    .filter(|c| *c == ch)
                    .count();
                if count >= min && count <= max {
                    ans += 1;
                }
            }
        }
        ans.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")
            .expect("Invalid regex pattern");

        let mut ans = 0;

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let min = caps[1].parse::<usize>().unwrap();
                let max = caps[2].parse::<usize>().unwrap();
                let ch = caps[3].chars().next().unwrap();
                let s = &caps[4];

                let min_char = s.chars().nth(min - 1).unwrap();
                let max_char = s.chars().nth(max - 1).unwrap();

                if (max_char == ch || min_char == ch) && max_char != min_char {
                    ans += 1;
                }
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    const TEST_INPUT_P2: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "1");
    }
}

