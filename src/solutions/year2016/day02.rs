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

const KEYPAD_1: [[char; 3]; 3] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
];

const KEYPAD_2: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

impl Day02 {
    pub fn part1(&self, input: &str) -> String {
        struct Position {
            x: i32,
            y: i32,
        }

        impl Position {
            fn new(x: i32, y: i32) -> Self {
                Self { x, y }
            }

            fn move_player(&mut self, dir: char) {
                let (dx, dy) = match dir {
                    'U' => (-1, 0),
                    'D' => (1, 0),
                    'L' => (0, -1),
                    'R' => (0, 1),
                    _ => (0, 0),
                };

                let new_x = self.x + dx;
                let new_y = self.y + dy;

                if Self::is_valid(new_x, new_y) {
                    self.x = new_x;
                    self.y = new_y;
                }
            }

            fn is_valid(x: i32, y: i32) -> bool {
                (0..=2).contains(&x) && (0..=2).contains(&y)
            }
        }

        let mut pos = Position::new(1, 1);
        let mut ans = String::new();

        for line in input.lines() {
            for dir in line.chars() {
                pos.move_player(dir);
            }
            let digit = KEYPAD_1[pos.x as usize][pos.y as usize];
            ans.push(digit);
        }

        ans
    }

    pub fn part2(&self, input: &str) -> String {
        struct Position {
            x: i32,
            y: i32,
        }

        impl Position {
            fn new(x: i32, y: i32) -> Self {
                Self { x, y }
            }

            fn move_player(&mut self, dir: char) {
                let (dx, dy) = match dir {
                    'U' => (-1, 0),
                    'D' => (1, 0),
                    'L' => (0, -1),
                    'R' => (0, 1),
                    _ => (0, 0),
                };

                let new_x = self.x + dx;
                let new_y = self.y + dy;

                if Self::is_valid(new_x, new_y) {
                    self.x = new_x;
                    self.y = new_y;
                }
            }

            fn is_valid(x: i32, y: i32) -> bool {
                (0..=4).contains(&x) && (0..=4).contains(&y)
                && KEYPAD_2[x as usize][y as usize] != ' '
            }
        }

        let mut pos = Position::new(2, 0);
        let mut ans = String::new();

        for line in input.lines() {
            for dir in line.chars() {
                pos.move_player(dir);
            }
            let ch = KEYPAD_2[pos.x as usize][pos.y as usize];
            ans.push(ch);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "ULL\nRRDDD\nLURDL\nUUUUD";
    const TEST_INPUT_P2: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT_P1), "1985");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT_P2), "5DB3");
    }
}

