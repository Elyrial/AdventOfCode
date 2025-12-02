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

// Part 1
fn is_repeated(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 { return false; }

    // Split the string in the middle and check if they match
    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);
    left == right
}

// Part 2
fn is_repeated_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible block lengths that could repeat at least twice.
    // A block can't be longer than half the string.
    for block_len in 1..=(len / 2) {

        // The total length must be divisible by the block length.
        if len % block_len != 0 {
            continue;
        }

        // Use the first block as the candidate pattern.
        let block = &s[..block_len];
        let mut ok = true;

        // Check every subsequent block against the first one.
        for i in (block_len..len).step_by(block_len) {
            if &s[i..i + block_len] != block {
                ok = false;
                break;
            }
        }

        // If all blocks match, it's a repeated pattern
        if ok {
            return true;
        }
    }
    false
}

impl Day02 {
    pub fn part1(&self, input: &str) -> String {
        input
            .split(',')
            .map(|range| {
                let (a, b) = range.split_once('-').unwrap();
                let start: u64 = a.parse().unwrap();
                let end: u64 = b.parse().unwrap();
                (start..=end)
                    .filter(|&n| is_repeated(n))
                    .sum::<u64>()
            })
            .sum::<u64>()
            .to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        input
            .split(',')
            .map(|range| {
                let (a, b) = range.split_once('-').unwrap();
                let start: u64 = a.parse().unwrap();
                let end: u64 = b.parse().unwrap();
                (start..=end)
                    .filter(|&n| is_repeated_at_least_twice(n))
                    .sum::<u64>()
            })
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT), "4174379265");
    }
}


