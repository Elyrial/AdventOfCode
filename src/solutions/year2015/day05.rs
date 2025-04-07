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
    pub fn part1(&self, input: &str) -> String {
        let forbidden_substrings = ["ab", "cd", "pq", "xy"];
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        input.lines()
            .filter(|line| {
                // Check forbidden substrings
                !forbidden_substrings.iter().any(|&sub| line.contains(sub))
            })
            .filter(|line| {
                // Check for double letters
                line.as_bytes().windows(2).any(|pair| pair[0] == pair[1])
            })
            .filter(|line| {
                // Count vowels
                line.chars().filter(|c| vowels.contains(c)).count() >= 3
            })
            .count()
            .to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        input.lines()
            .filter(|line| {
                // Check for non overlapping pair appering twice
                (0..line.len().saturating_sub(1)).any(|i| {
                    let pair = &line[i..i + 2];
                    line[i + 2..].contains(pair)
                })
            })
            .filter(|line| {
                // repeating letters with one letter between
                line.as_bytes().windows(3).any(|rep| rep[0] == rep[2])
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "ugknbfddgicrmopn\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb";
    const TEST_INPUT_P2: &str = "qjhvhtzxzqqjkmpb\nuurcxstgmygtbstg\nieodomkazucvgmuy";

    #[test]
    fn test_part1() {
        assert_eq!(Day05.part1(TEST_INPUT_P1), "1");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day05.part2(TEST_INPUT_P2), "1");
    }
}

