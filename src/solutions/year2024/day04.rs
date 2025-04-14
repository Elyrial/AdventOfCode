use std::time::{Duration, Instant};

pub struct Day04;

impl super::super::Solution for Day04 {
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

fn check_direction(grid: &[Vec<char>], dx: i32, dy: i32) -> i32 {
    let mut dir_count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for x in 0..rows {
        for y in 0..cols {
            // Is valid?
            let x3 = x as i32 + dx * 3;
            let y3 = y as i32 + dy * 3;
            if x3 < 0 || x3 >= cols as i32 || y3 < 0 || y3 >= rows as i32 {
                continue;
            }

            let (x, y) = (x as usize, y as usize);
            let x1 = (x as i32 + dx) as usize;
            let y1 = (y as i32 + dy) as usize;
            let x2 = (x as i32 + dx * 2) as usize;
            let y2 = (y as i32 + dy * 2) as usize;
            let x3 = x3 as usize;
            let y3 = y3 as usize;

            if grid[x][y] == 'X' && grid[x1][y1] == 'M' && grid[x2][y2] == 'A' && grid[x3][y3] == 'S' {
                dir_count += 1;
            }
        }
    }
    dir_count
}
    

impl Day04 {
    pub fn part1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines()
            .map(|s| s.chars().collect())
            .collect();

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];

        let count: i32 = directions.iter()
            .map(|&(dx, dy)| check_direction(&grid, dx, dy))
            .sum();

        count.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines()
            .map(|s| s.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                if grid[i][j] == 'A' {
                    let first_pattern = (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
                                     || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M');

                    let second_pattern = (grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S')
                                      || (grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M');

                    if first_pattern && second_pattern {
                        count += 1;
                    }
                }
            }
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(Day04.part1(TEST_INPUT), "18");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04.part2(TEST_INPUT), "9");
    }
}

