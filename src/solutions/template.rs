// TODO: rename DayXX to match the actual day number (e.g. Day03)
pub struct DayXX;

crate::impl_solution!(DayXX);

impl DayXX {
    pub fn part1(&self, input: &str) -> String {
        input.lines().count().to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        input.lines().count().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "";
    const TEST_INPUT_P2: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(DayXX.part1(TEST_INPUT_P1), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(DayXX.part2(TEST_INPUT_P2), "");
    }
}
