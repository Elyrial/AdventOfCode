use std::collections::{HashSet, HashMap};

pub struct Day07;

crate::impl_solution!(Day07);

fn find_start_col(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter().enumerate())
        .find_map(|(c, &ch)| if ch == 'S' { Some(c) } else { None })
        .unwrap()
}

impl Day07 {
    pub fn part1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        let h = grid.len();
        let start_col = find_start_col(&grid);

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
                    new_beams.insert(c - 1);
                    new_beams.insert(c + 1);

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
        let start_col = find_start_col(&grid);

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

