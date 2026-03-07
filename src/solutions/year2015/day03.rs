use std::collections::HashSet;

pub struct Day03;

crate::impl_solution!(Day03);

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

