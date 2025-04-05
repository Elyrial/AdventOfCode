use std::collections::HashSet;
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

        let mut x = 0;
        let mut y = 0;
        let mut dir = 0; // 0=N, 1=E, 2=S, 3=W

        // Direction vectors: N, E, S, W
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for instruction in input.trim().split(", ") {
            let (turn, steps) = instruction.split_at(1);
            let steps: i32 = steps.parse().unwrap();

            dir = match turn {
                "R" => (dir + 1) % 4,
                "L" => (dir + 3) % 4,
                _ => panic!("Invalid turn"),
            };

            let (dx, dy) = directions[dir];
            x += dx * steps;
            y += dy * steps;
        }

        (x.abs() + y.abs()).to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut dir = 0; // 0=N, 1=E, 2=S, 3=W

        // Direction vectors: N, E, S, W
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut visited = HashSet::new();
        visited.insert((x, y)); // Insert starting position

        for instruction in input.trim().split(", ") {
            let (turn, steps) = instruction.split_at(1);
            let steps: i32 = steps.parse().unwrap();

            dir = match turn {
                "R" => (dir + 1) % 4,
                "L" => (dir + 3) % 4,
                _ => panic!("Invalid turn"),
            };

            let (dx, dy) = directions[dir];

            for _ in 0..steps {
                x += dx;
                y += dy;
                if !visited.insert((x, y)) {
                    return (x.abs() + y.abs()).to_string();
                }
            }
        }
        "No location was found twice".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "R5, L5, R5, R3:";
    const TEST_INPUT_P2: &str = "R8, R4, R4, R8";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "12");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "4");
    }
}


