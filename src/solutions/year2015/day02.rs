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
        let mut amount = 0;
        for equation in input.lines() {
            let line: Vec<i32> = equation.split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let side1 = line[0] * line[1];
            let side2 = line[1] * line[2];
            let side3 = line[0] * line[2];

            let slack = side1.min(side2).min(side3);

            amount += 2 * side1 + 2 * side2 + 2 * side3 + slack;

        }

       amount.to_string() 
    }

    pub fn part2(&self, input: &str) -> String {
        let mut amount = 0;
        for equation in input.lines() {
            let mut line: Vec<i32> = equation.split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            line.sort();

            let wrap = 2 * (line[0] + line[1]);
            let ribbon: i32 = line.iter().product();

            amount += wrap + ribbon;
        }
        amount.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "2x3x4";
    const TEST_INPUT_P2: &str = "2x3x4";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "58");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "34");
    }
}

