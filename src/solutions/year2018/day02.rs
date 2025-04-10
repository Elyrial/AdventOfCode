use std::collections::HashMap;
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
        let (mut twos, mut threes) = (0, 0);

        for line in input.lines() {
            let mut counter = HashMap::new();

            for ch in line.chars() {
                *counter.entry(ch).or_insert(0) += 1;
            }

            let has_two = counter.values().any(|&v| v == 2);
            let has_three = counter.values().any(|&v| v == 3);

            if has_two {
                twos += 1;
            }
            if has_three {
                threes += 1;
            }
        }

        (twos * threes).to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let ids: Vec<String> = input.lines()
            .map(|s| s.to_string())
            .collect();

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let s1 = &ids[i];
                let s2 = &ids[j];

                let diff_count = s1.chars()
                    .zip(s2.chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count();

                if diff_count == 1 {
                    let common_string: String = s1.chars()
                        .zip(s2.chars())
                        .filter(|(c1, c2)| c1 == c2)
                        .map(|(c, _)| c)
                        .collect();

                    return common_string;
                }
            }
        }
        "Not found".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    const TEST_INPUT_P2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "12");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "fgij");
    }
}

