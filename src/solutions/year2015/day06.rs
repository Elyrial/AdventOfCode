use regex::Regex;

pub struct Day06;

crate::impl_solution!(Day06);

impl Day06 {
    pub fn part1(&self, input: &str) -> String {
        let mut matrix = vec![vec![0; 1000]; 1000];
        let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")
            .expect("Invalid regex pattern");

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let action = caps.get(1).unwrap().as_str();
                let x1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                let y1: usize = caps.get(3).unwrap().as_str().parse().unwrap();
                let x2: usize = caps.get(4).unwrap().as_str().parse().unwrap();
                let y2: usize = caps.get(5).unwrap().as_str().parse().unwrap();

                for i in x1..x2 + 1 {
                    for j in y1..y2 + 1 {
                        match action {
                            "turn on" => matrix[i][j] = 1,
                            "turn off" => matrix[i][j] = 0,
                            "toggle" => matrix[i][j] ^= 1,
                            _ => (),
                        }
                    }
                }
            }
        }

        let count = matrix.iter().flatten().sum::<i32>().to_string();
        count

    }

    pub fn part2(&self, input: &str) -> String {
        let mut matrix = vec![vec![0; 1000]; 1000];
        let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")
            .expect("Invalid regex pattern");

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let action = caps.get(1).unwrap().as_str();
                let x1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                let y1: usize = caps.get(3).unwrap().as_str().parse().unwrap();
                let x2: usize = caps.get(4).unwrap().as_str().parse().unwrap();
                let y2: usize = caps.get(5).unwrap().as_str().parse().unwrap();

                for i in x1..x2 + 1 {
                    for j in y1..y2 + 1 {
                        match action {
                            "turn on" => matrix[i][j] += 1,
                            "turn off" => if matrix[i][j] > 0 { matrix[i][j] -= 1 },
                            "toggle" => matrix[i][j] += 2,
                            _ => (),
                        }
                    }
                }
            }
        }

        let count = matrix.iter().flatten().sum::<i32>().to_string();
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "turn on 0,0 through 999,999";
    const TEST_INPUT_P2: &str = "turn on 0,0 through 999,999";

    #[test]
    fn test_part1() {
        assert_eq!(Day06.part1(TEST_INPUT_P1), "1000000");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06.part2(TEST_INPUT_P2), "1000000");
    }
}

