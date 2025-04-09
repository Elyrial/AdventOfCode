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
        let mut sum = 0;
        for line in input.lines() {
            let seq: Vec<i32> = line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let min = seq.iter().min().unwrap();
            let max = seq.iter().max().unwrap();

            sum += max - min;
        }
        sum.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let seq: Vec<i32> = line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            for i in 0..seq.len() {
                for j in 0..seq.len() {
                    if i != j && seq[i] % seq[j] == 0 {
                        sum += seq[i] / seq[j];
                        break;
                    }
                }
            }
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "5 1 9 5\n7 5 3\n2 4 6 8";
    const TEST_INPUT_P2: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "18");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "9");
    }
}

