use std::time::{Duration, Instant};

pub struct Day03;

impl super::super::Solution for Day03 {
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

impl Day03 {
    pub fn part1(&self, input: &str) -> String {
        let count: usize = input.lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .map(|mut nums| {
                nums.sort();
                (nums[0], nums[1], nums[2])
            })
            .filter(|&(a, b, c)| a + b > c)
            .count();

        count.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let matrix: Vec<Vec<usize>> = input.lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect();

        let cols = matrix.first().map_or(0, |row| row.len());

        let ans = (0..cols)
            .map(|col| {
                matrix.iter()
                    .map(|row| row[col])
                    .collect::<Vec<usize>>()
            })
            .map(|column| {
                column.chunks(3)
                    .filter(|&chunk| {
                        let mut sides = chunk.to_vec();
                        sides.sort();
                        let (a, b, c) = (sides[0], sides[1], sides[2]);
                        a + b > c
                    })
                    .count()
            })
            .sum::<usize>();

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "233 581 299\n340 306 308\n382 100 200";
    const TEST_INPUT_P2: &str = "233 581 299\n304 306 308\n382 100 200";

    #[test]
    fn test_part1() {
        assert_eq!(Day03.part1(TEST_INPUT_P1), "1");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03.part2(TEST_INPUT_P2), "2");
    }
}

