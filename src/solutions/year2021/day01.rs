pub struct Day01;

crate::impl_solution!(Day01);

impl Day01 {
    pub fn part1(&self, input: &str) -> String {
        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        let ans = numbers.windows(2)
            .filter(|pair| pair[0] < pair[1])
            .count()
            .to_string();
        
        ans
    }

    pub fn part2(&self, input: &str) -> String {
        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        let ans = numbers.windows(4)
            .filter(|w| w[0] < w[3]) // Same as (w[0] + w[1] + w[2] < w[1] + w[2] + w[3])
            .count()
            .to_string();
        
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    const TEST_INPUT_P2: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "5");
    }
}

