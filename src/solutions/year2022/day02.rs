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
        let total: i32 = input.lines()
            .map(|line| {
                let bytes = line.as_bytes();
                (bytes[0] as char, bytes[2] as char)
            })
            .map(|(p1, p2)| match (p1, p2) {
                ('A', 'X') => 4,
                ('A', 'Y') => 8,
                ('A', 'Z') => 3,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 7,
                ('C', 'Y') => 2,
                ('C', 'Z') => 6,
                _ => 0,
            })
            .sum::<i32>();
        total.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let total: i32 = input.lines()
            .map(|line| {
                let bytes = line.as_bytes();
                (bytes[0] as char, bytes[2] as char)
            })
            .map(|(p1, p2)| match (p1, p2) {
                ('A', 'X') => 3,
                ('A', 'Y') => 4,
                ('A', 'Z') => 8,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 2,
                ('C', 'Y') => 6,
                ('C', 'Z') => 7,
                _ => 0,
            })
            .sum::<i32>();
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "A Y\nB X\nC Z";
    const TEST_INPUT_P2: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "15");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "12");
    }
}

