use regex::Regex;
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

fn is_possible_game(game: &str) -> bool {
    for round in game.split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for part in round.split(", ") {
            let part = part.trim(); // Took me hours to find...
            if let Some((count, color)) = part.split_once(' ') {
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => (),
                }
            }
        }
        if red > 12 || green > 13 || blue > 14 {
            return false;
        }
    }
    true
}

fn sum_of_products(game: &str) -> u32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for round in game.split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for part in round.split(", ") {
            let part = part.trim(); // Took me hours to find...
            if let Some((count, color)) = part.split_once(' ') {
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" => red = count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => (),
                }
            }
        }
        max_red = max_red.max(red);
        max_green = max_green.max(green);
        max_blue = max_blue.max(blue);
    }
    max_red * max_green * max_blue
}


impl Day02 {
    pub fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"Game (\d+): (.+)")
            .expect("Invalid regex pattern");

        let sum: u32 = input.lines()
            .filter_map(|line| {
                let caps = re.captures(line)?;
                let id = caps.get(1)?.as_str().parse::<u32>().ok()?;
                let game = caps.get(2)?.as_str();

                is_possible_game(game).then_some(id)
            })
            .sum();
            
        sum.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"Game (\d+): (.+)")
            .expect("Invalid regex pattern");

        let sum: u32 = input.lines()
            .filter_map(|line| {
                let caps = re.captures(line)?;
                let game = caps.get(2)?.as_str();

                Some(sum_of_products(game))
            })
            .sum();
            
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT), "8");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT), "2286");
    }
}

