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
        let (mut x, mut y) = (0, 0);
        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let dir = parts.next().expect("Missing direction");
            let units = parts.next().expect("Missing units").parse::<i32>().unwrap();
            match dir {
                "forward" => x += units,
                "down" => y += units,
                "up" => y -= units,
                _ => return "Invalid direction".to_string(),
            }
        }

        (x * y).to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let (mut x, mut y, mut aim) = (0, 0, 0);
        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let dir = parts.next().expect("Missing direction");
            let units = parts.next().expect("Missing units").parse::<i32>().unwrap();
            match dir {
                "forward" => {
                    x += units;
                    y += aim * units;
                },
                "down" => aim += units,
                "up" => aim -= units,
                _ => return "Invalid direction".to_string(),
            }
        }

        (x * y).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    const TEST_INPUT_P2: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "150");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "900");
    }
}

