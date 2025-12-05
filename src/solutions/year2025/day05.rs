use std::time::{Duration, Instant};

pub struct Day05;

impl super::super::Solution for Day05 {
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

impl Day05 {
    // Count how many of the available ingredient IDs are in the fresh range
    pub fn part1(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");

        // Collect ranges in a vector
        let ranges: Vec<(u64, u64)> = parts.next().unwrap()
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
            })
        .collect::<Vec<(u64, u64)>>();

        // Parse IDs
        let ids = parts.next().unwrap()
            .lines()
            .map(|l| l.parse::<u64>().unwrap());

        // Count IDs that fit any of the ranges
        let fresh = ids.filter(|id| {
            ranges.iter().any(|(s, e)| id >= s && id <= e)
        }).count();

        fresh.to_string()
    }

    // Count all distinct ingredient IDs covered by the fresh ranges
    pub fn part2(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");

        // Collect ranges in a vector
        let mut ranges: Vec<(u64, u64)> = parts.next().unwrap()
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
            })
        .collect::<Vec<(u64, u64)>>();

        // Sort ranges by start
        ranges.sort_by_key(|&(start, _end)| start);

        // Initialize the first merged interval
        let (mut merged_start, mut merged_end) = ranges[0];
        let mut total_covered = 0;

        // Iterate over all ranges, skip first since its used for init
        for (start, end) in ranges.into_iter().skip(1) {
            if start <= merged_end + 1 {

                // Overlapping or adjectent range extends the current merged interval
                if end > merged_end {
                    merged_end = end;
                }

            } else {
                // Non-overlapping ranges finalizes the range
                total_covered += merged_end - merged_start + 1;

                // Start a new merged interval
                merged_start = start;
                merged_end = end;
            }
        }

        // Add the last interval after finishing the loop
        total_covered += merged_end - merged_start + 1;

        total_covered.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(Day05.part1(TEST_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day05.part2(TEST_INPUT), "14");
    }
}

