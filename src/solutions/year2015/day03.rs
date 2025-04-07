use std::collections::HashSet;
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
        let mut visited = HashSet::new();
        let (mut x, mut y) = (0, 0);
        visited.insert((x, y));

        for c in input.chars() {
            let (dx, dy) = match c {
                '^' => (0, 1),
                '>' => (1, 0),
                'v' => (0, -1),
                '<' => (-1, 0),
                _ => continue,
            };
            x += dx;
            y += dy;
            visited.insert((x, y));
        }
        visited.len().to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut visited = HashSet::new();
        let (mut x, mut y, mut rx, mut ry) = (0, 0, 0, 0);
        visited.insert((x, y));

        for (i, c) in input.chars().enumerate() {
            let (dx, dy) = match c {
                '^' => (0, 1),
                '>' => (1, 0),
                'v' => (0, -1),
                '<' => (-1, 0),
                _ => continue,
            };
            if i % 2 == 0 {
                x += dx;
                y += dy;
                visited.insert((x, y));
            } else {
                rx += dx;
                ry += dy;
                visited.insert((rx, ry));
            }
        }
        visited.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "^v^v^v^v^v";
    const TEST_INPUT_P2: &str = "^v^v^v^v^v";

    #[test]
    fn test_part1() {
        assert_eq!(Day03.part1(TEST_INPUT_P1), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03.part2(TEST_INPUT_P2), "11");
    }
}

