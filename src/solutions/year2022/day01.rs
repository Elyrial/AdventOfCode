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
        let sums: Vec<i32> = input.split("\n\n")
            .map(|group| {
                group.lines()
                    .filter_map(|line| line.parse::<i32>().ok())
                    .sum()
            })
            .collect();

        let max = sums.iter().max().unwrap().to_string();
        max
    }

    pub fn part2(&self, input: &str) -> String {
        let mut sums: Vec<i32> = input.split("\n\n")
            .map(|group| {
                group.lines()
                    .filter_map(|line| line.parse::<i32>().ok())
                    .sum()
            })
            .collect();

        sums.sort_by(|a, b| b.cmp(a));

        let top3_sum = sums.iter().take(3).sum::<i32>();
        top3_sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    const TEST_INPUT_P2: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "24000");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "45000");
    }
}

