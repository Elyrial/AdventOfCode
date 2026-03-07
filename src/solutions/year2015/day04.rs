use md5;

pub struct Day04;

crate::impl_solution!(Day04);

impl Day04 {
    pub fn part1(&self, input: &str) -> String {
        let mut num = 1;

        loop {
            let test_input = format!("{}{}", input, num);
            let hash = md5::compute(test_input.as_bytes());
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with("00000") {
                return num.to_string();
            }
            num += 1;
        }
    }

    pub fn part2(&self, input: &str) -> String {
        let mut num = 1;

        loop {
            let test_input = format!("{}{}", input, num);
            let hash = md5::compute(test_input.as_bytes());
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with("000000") {
                return num.to_string();
            }
            num += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "bgvyzdsv";
    const TEST_INPUT_P2: &str = "bgvyzdsv";

    #[test]
    fn test_part1() {
        assert_eq!(Day04.part1(TEST_INPUT_P1), "254575");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04.part2(TEST_INPUT_P2), "1038736");
    }
}

