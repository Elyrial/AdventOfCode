use std::collections::{HashSet, HashMap};
use std::time::{Duration, Instant};

pub struct Day05;

impl super::super::Solution for Day05 {
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

impl Day05 {
    pub fn part1(&self, input: &str) -> String {
        let mut sum = 0;
        let (instructions, sequences) = input.split_once("\n\n").unwrap();
        let mut order = HashMap::<usize, HashSet<usize>>::new();

        for instruction in instructions.lines() {
            let (a, b) = instruction.split_once('|').unwrap();
            order.entry(b.parse().unwrap()).or_default().insert(a.parse().unwrap());
        }

        let pages = sequences.lines()
            .map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());

        for page in pages {
            if page.is_sorted_by(|x, y| order[y].contains(x)) {
                sum += page[page.len() / 2];
            }
        }

        sum.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut sum = 0;
        let (instructions, sequences) = input.split_once("\n\n").unwrap();
        let mut order = HashMap::<usize, HashSet<usize>>::new();

        for instruction in instructions.lines() {
            let (a, b) = instruction.split_once('|').unwrap();
            order.entry(b.parse().unwrap()).or_default().insert(a.parse().unwrap());
        }

        let pages = sequences.lines()
            .map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());

        for mut page in pages {
            if !page.is_sorted_by(|x, y| order[y].contains(x)) {
                page.sort_by(|x, y| order[y].contains(x).cmp(&true));
                sum += page[page.len() / 2];
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(Day05.part1(TEST_INPUT), "143");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day05.part2(TEST_INPUT), "123");
    }
}

