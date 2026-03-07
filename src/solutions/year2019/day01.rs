pub struct Day01;

crate::impl_solution!(Day01);

impl Day01 {
    pub fn part1(&self, input: &str) -> String {
        let mut total_fuel = 0;

        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for num in numbers.iter() {
            total_fuel += num / 3 - 2;
        }

        total_fuel.to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let mut total_fuel = 0;

        let numbers: Vec<i32> = input.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();

        for &module in &numbers {
            let mut current_mass = module;
            
            loop {
                let fuel = current_mass / 3 - 2;
                if fuel <= 0 {
                    break;
                }
                total_fuel += fuel;
                current_mass = fuel;
            }
        }

        total_fuel.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "100756";
    const TEST_INPUT_P2: &str = "100756";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT_P1), "33583");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT_P2), "50346");
    }
}


