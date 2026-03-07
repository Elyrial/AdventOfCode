use std::collections::HashSet;

pub struct Day01;

crate::impl_solution!(Day01);

impl Day01 {
    pub fn part1(&self, input: &str) -> String {
        input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .sum::<i32>()
            .to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut freq = 0;
        let mut visited = HashSet::new();
        visited.insert(freq);

        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for num in numbers.iter().cycle() {
            freq += num;
            if !visited.insert(freq) {
                return freq.to_string();
            }
        }

        unreachable!("A cycle iterator never ends");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "+1\n-2\n+3\n+1";
    const TEST_INPUT_P2: &str = "+7\n+7\n-2\n-7\n-4";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "14");
    }
}

