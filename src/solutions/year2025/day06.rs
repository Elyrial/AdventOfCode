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

fn parse_numbers(input: &str) -> Vec<Vec<u128>> {
    let lines: Vec<&str> = input.lines().collect();
    lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
            .map(|n| n.parse::<u128>().unwrap())
            .collect()
        })
        .collect()
}

fn parse_operators(input: &str) -> Vec<char> {
    input.lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect()
}

impl Day06 {
    pub fn part1(&self, input: &str) -> String {
        let numbers = parse_numbers(input);
        let operators = parse_operators(input);

        let mut total = 0u128;
        let num_problems = operators.len();

        for col in 0..num_problems {
            let mut col_value = numbers[0][col];
            for row in 1..numbers.len() {
                match operators[col] {
                    '+' => col_value += numbers[row][col],
                    '*' => col_value *= numbers[row][col],
                    _ => {}
                }
            }
            total += col_value;
        }

        total.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines[0].len();

        // Collect columns as strings
        let mut columns: Vec<String> = vec![String::new(); width];
        for line in &lines {
            for (i, c) in line.chars().enumerate() {
                columns[i].push(c);
            }
        }

        let mut total = 0u128;
        let mut current_sum = 0u128;
        let mut current_op = '+';

        for col in columns {
            let trimmed = col.trim();

            if trimmed.ends_with('+') || trimmed.ends_with('*') {
                total += current_sum;
                current_op = trimmed.chars().last().unwrap();
                current_sum = match current_op {
                    '+' => 0,
                    '*' => 1,
                    _ => 0,
                };
            }

            let num_str: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
            if !num_str.is_empty() {
                let num = num_str.parse::<u128>().unwrap_or(0);
                current_sum = match current_op {
                    '+' => current_sum + num,
                    '*' => current_sum * num,
                    _ => current_sum,
                };
            }
        }

        total += current_sum;
        total.to_string()
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

