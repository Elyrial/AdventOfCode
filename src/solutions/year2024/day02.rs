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

fn is_safe_report(seq: &[i32]) -> bool {

    let diffs: Vec<i32> = seq.windows(2).map(|w| w[1] - w[0]).collect();

    let is_valid_diff = diffs.iter().all(|&d| d.abs() >= 1 && d.abs() <= 3);
    let is_monotonic = diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0);

    is_valid_diff && is_monotonic
}
    

impl Day02 {
    pub fn part1(&self, input: &str) -> String {
        let safe_sequences = input.lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .filter(|seq| is_safe_report(seq))
            .count();

        safe_sequences.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let safe_sequences = input.lines()
            .map(|line| {
                let original: Vec<i32> = line.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect();

                (0..original.len()).any(|i| {
                    let mut modified = original.clone();
                    modified.remove(i);
                    is_safe_report(&modified)
                })
            })
            .filter(|&is_safe| is_safe)
            .count();

        safe_sequences.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT), "4");
    }
}

