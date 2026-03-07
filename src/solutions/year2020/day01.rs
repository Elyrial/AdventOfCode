pub struct Day01;

crate::impl_solution!(Day01);

impl Day01 {
    pub fn part1(&self, input: &str) -> String {
        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for i in &numbers {
            for j in &numbers {
                if i + j == 2020 {
                    return (i * j).to_string();
                }
            }
        }
        "Not found!".to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for i in &numbers {
            for j in &numbers {
                for h in &numbers {
                    if i + j + h == 2020 {
                        return (i * j * h).to_string();
                    }
                }
            }
        }
        "Not found!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1721\n979\n366\n299\n675\n1456";
    const TEST_INPUT_P2: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "514579");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "241861950");
    }
}

