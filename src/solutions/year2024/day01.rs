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
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        for pair in input.lines() {
            let mut parts = pair.split_whitespace();
            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                if let (Ok(num_a), Ok(num_b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                    v1.push(num_a);
                    v2.push(num_b);
                }
            }
        }

        v1.sort();
        v2.sort();

        let ans: i32 = v1.iter()
            .zip(v2.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum::<i32>();
        ans.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        for pair in input.lines() {
            let mut parts = pair.split_whitespace();
            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                if let (Ok(num_a), Ok(num_b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                    v1.push(num_a);
                    v2.push(num_b);
                }
            }
        }

        let mut sum = 0;
        for num in v1 {
            let count = v2.iter().filter(|x| **x == num).count() as i32;
            sum += num * count;
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
    const TEST_INPUT_P2: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "31");
    }
}

