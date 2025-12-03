use std::time::{Duration, Instant};

pub struct Day03;

impl super::super::Solution for Day03 {
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

impl Day03 {
    pub fn part1(&self, input: &str) -> String {
        let mut total = 0;

        for line in input.lines() {
           // 'b - b'0'' converts the ASCII byte to a numeric value.
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

            let mut best = 0;

            // Iterate through all possible combinations and find the best value
            for i in 0..digits.len() {
                for j in i+1..digits.len() {
                    let val = digits[i] as i32 * 10 + digits[j] as i32;
                    if val > best {
                        best = val;
                    }
                }
            }

            total += best;
        }

        total.to_string()
    }

    // My stack-based greedy algorithm idea:
    //
    // We want exaclty 12 digits for tha largest number in order.
    // For each digit, we consider if it's worth replacing the last digit in the current stack.
    // - Only replace if the current digit is larger than the last one.
    // - Only replace if we can still pick enough digits from the remaining digits to reach 12.
    // If the stack isn't full, add the current digit.
    pub fn part2(&self, input: &str) -> String {
       let mut total: u128 = 0;
       let k = 12;

       for line in input.lines() {
           // 'b - b'0'' converts the ASCII byte to a numeric value.
           let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
           let n = digits.len();
           let mut stack: Vec<u8> = Vec::new();

           for (i, &d) in digits.iter().enumerate() {
               // While there is a digit in the stack:
               // 1. The last digit in the stack is smaller than the current digit
               // 2. There are enough remaining digits to still reach k digits
               // Then, we pop the smaller digit from the stack to make room for a larger digit
               while let Some(&last) = stack.last() {
                   if last < d && stack.len() + (n - i - 1) >= k {
                       stack.pop(); // Remove smaller digit to allow a bigger one
                   } else {
                       break;
                   }
               }

               // If the stack hasn't picked k digits yet, push the current digit
               if stack.len() < k {
                   stack.push(d);
               }
           }

           // Convert the selected 12 digits into an actual number
           // Iterate through the stack and build the number by multiplying by 10 and adding the
           // next digit
           let number: u128 = stack.iter().fold(0, |acc, &d| acc * 10 + d as u128);
           total += number;
       }
       total.to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(Day03.part1(TEST_INPUT), "357");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03.part2(TEST_INPUT), "3121910778619");
    }
}


