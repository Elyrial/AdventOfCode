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
        let mut dial: i32 = 50;
        let mut counter: i32 = 0;

        for instruction in input.lines() {
            let dir: char = instruction.chars().nth(0).unwrap();
            let value: i32 = instruction[1..].parse().unwrap();

            let delta = if dir == 'L' { -value } else { value };
            dial += delta;

            dial = ((dial % 100) + 100) % 100;

            if dial == 0 {
                counter += 1;
            }
        }
        counter.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut dial: i32 = 50;
        let mut counter: i32 = 0;

        for instruction in input.lines() {
            let dir: char = instruction.chars().nth(0).unwrap();
            let value: i32 = instruction[1..].parse().unwrap();

            if dir == 'R' {
                counter += (dial + value) / 100;
                dial = ((dial + value) % 100 + 100) % 100;
            } else {
                let t0: i32 = if dial == 0 { 100 } else { dial };
                if value >= t0 {
                    counter += 1 + (value - t0) / 100;
                }
                dial = ((dial - value) % 100 + 100) % 100;
            }
        }
        counter.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT), "6");
    }
}


