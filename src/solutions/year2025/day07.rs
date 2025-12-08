use std::time::{Duration, Instant};
use std::collections::{HashSet, HashMap};

pub struct Day07;

impl super::super::Solution for Day07 {
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

impl Day07 {
    pub fn part1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        let h = grid.len();

        // Find S
        let mut start_col = None;
        for (_r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    start_col = Some(c);
                    break;
                }
            }
        }
        let start_col = start_col.unwrap();

        let mut active_beams = HashSet::new();
        active_beams.insert(start_col);

        let mut splits = 0;

        // Process grid row by row
        for r in 1..h {
            let row = &grid[r];
            let mut new_beams = HashSet::new();

            for c in 0..row.len() {
                if row[c] == '^' && active_beams.contains(&c) {
                    // Beam hits a splitter
                    splits += 1;

                    // Create beam to the left and right
                    if c > 0 {
                        new_beams.insert(c - 1);
                    }
                    if c + 1 < row.len() {
                        new_beams.insert(c + 1);
                    }

                } else if active_beams.contains(&c) && row[c] != '^' {
                    // Continues downward if not a splitter
                    new_beams.insert(c);
                }
            }

            active_beams = new_beams;
        }
        
        splits.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        let h = grid.len();

        // Find S
        let mut start_col = None;
        for (_r, row) in grid.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    start_col = Some(c);
                    break;
                }
            }
        }
        let start_col = start_col.unwrap();

        // Map: column -> number of timelines currently
        let mut timelines = HashMap::new();
        timelines.insert(start_col, 1usize);

        for r in 1..h {
            let mut next = HashMap::new();
            for (&c, &count) in &timelines {
                match grid[r][c] {
                    '.' => { *next.entry(c).or_insert(0) += count; }
                    '^' => {
                        // Send counts to left and right
                        *next.entry(c - 1).or_insert(0) += count;
                        *next.entry(c + 1).or_insert(0) += count;
                    }
                    _ => { *next.entry(c).or_insert(0) += count; }
                }
            }
            timelines = next;
        }
        timelines.values().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1() {
        assert_eq!(Day07.part1(TEST_INPUT), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day07.part2(TEST_INPUT), "40");
    }
}

