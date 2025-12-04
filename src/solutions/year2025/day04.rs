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

// Helper function to check if a coordinate is outside the grid
fn outside_bounds(nr: isize, nc: isize, h: usize, w: usize) -> bool {
    if nr < 0 || nc < 0 {
        return true;
    }
    let (ur, uc) = (nr as usize, nc as usize);
    ur >= h || uc >= w
}

// Count how many adjecent '@' cells a given cell has
fn count_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    // Offset for all 8 adjecent neighbors.
    let dirs = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut count = 0;

    // Check all 8 directions around this cell
    for (dr, dc) in dirs {
        let nr = row as isize + dr; // neighbor row
        let nc = col as isize + dc; // neighbor col

        // Ignore neighbors outside the grid
        if outside_bounds(nr, nc, h, w) {
            continue;
        }

        // Count the adjecent '@'
        if grid[nr as usize][nc as usize] == '@' {
            count += 1;
        }
    }

    count
}

// Determines if a cell is accessible (fewer than 4 adjecent '@')
fn is_accessible(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    grid[row][col] == '@' && count_neighbors(grid, row, col) < 4
}

// Returns a list of coordinates of all accessible rolls in the grid
fn find_accessible(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let h = grid.len();
    let w = grid[0].len();

    let mut result = Vec::new();

    for row in 0..h {
        for col in 0..w {
            if is_accessible(grid, row, col) {
                result.push((row, col));
            }
        }
    }
    result
}

impl Day04 {
    pub fn part1(&self, input: &str) -> String {
        // Convert ASCII map into a 2D char grid.
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        find_accessible(&grid).len().to_string()

    }

    pub fn part2(&self, input: &str) -> String {
        // Convert ASCII map into a 2D char grid.
        let mut grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut total_removed = 0;

        // Repeat removal until no accessible rools remain
        loop {
            let to_remove = find_accessible(&grid);

            if to_remove.is_empty() {
                break;
            }

            for (row, col) in to_remove.iter() {
                grid[*row][*col] = '.';
            }

            total_removed += to_remove.len();
        }
        total_removed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(Day04.part1(TEST_INPUT), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04.part2(TEST_INPUT), "43");
    }
}

