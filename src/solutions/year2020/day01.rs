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
        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for i in &numbers {
            for j in &numbers {
                if i + j == 2020 {
                    return (i * j).to_string();
                }
            }
        }
        "Not found!".to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for i in &numbers {
            for j in &numbers {
                for h in &numbers {
                    if i + j + h == 2020 {
                        return (i * j * h).to_string();
                    }
                }
            }
        }
        "Not found!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1721\n979\n366\n299\n675\n1456";
    const TEST_INPUT_P2: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "514579");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "241861950");
    }
}

