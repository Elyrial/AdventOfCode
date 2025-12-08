use std::time::{Duration, Instant};

pub struct Day06;

impl super::super::Solution for Day06 {
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

impl Day06 {
    pub fn part1(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines(). collect();

        // Collect the numbers
        let nums: Vec<Vec<u128>> = lines[..lines.len()-1]
            .iter()
            .map(|line| line.split_whitespace()
                .map(|n| n.parse::<u128>().unwrap())
                .collect())
            .collect();

        // Collect the operators
        let ops: Vec<char> = lines.last().unwrap()
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect();

        let mut total = 0u128;
        let num_problems = ops.len();

        for c in 0..num_problems {
            let mut val = nums[0][c];
            for r in 1..nums.len() {
                match ops[c] {
                    '+' => val += nums[r][c],
                    '*' => val *= nums[r][c],
                    _ => {},
                }
            }
            total += val;
        }

        total.to_string()

    }

    pub fn part2(&self, input: &str) -> String {
        // Implementation will follow
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(Day06.part1(TEST_INPUT), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06.part2(TEST_INPUT), "3263827");
    }
}

