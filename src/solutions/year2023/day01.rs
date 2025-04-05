use std::collections::HashMap;
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
        input.lines()
            .map(|line| {
                let digits: Vec<char> = line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect();

                match (digits.first(), digits.last()) {
                    (Some(first), Some(last)) => {
                        format!("{}{}", first, last).parse::<u32>().unwrap()
                    },
                    _ => 0,
                }
            })
            .sum::<u32>()
            .to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let word_to_digit: HashMap<&str, char> = [
            ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
            ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')
        ].iter().cloned().collect();

        input.lines()
            .map(|line| {
                let mut digits = Vec::new();

                for (i, c) in line.char_indices() {
                    if c.is_ascii_digit() {
                        digits.push(c);
                    } else {
                        for (word, &digit) in &word_to_digit {
                            if line[i..].starts_with(word) {
                                digits.push(digit);
                                break;
                            }
                        }
                    }
                }

                match (digits.first(), digits.last()) {
                    (Some(first), Some(last)) => {
                        format!("{}{}", first, last).parse::<u32>().unwrap()
                    },
                    _ => 0,
                }
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const TEST_INPUT_P2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "142");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "281");
    }
}
