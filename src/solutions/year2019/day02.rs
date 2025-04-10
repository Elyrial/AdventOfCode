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

const TARGET: i32 = 19690720;

fn run_program(memory: &mut Vec<i32>, noun: Option<i32>, verb: Option<i32>) -> i32 {

    if let Some(n) = noun {
        memory[1] = n;
    }
    if let Some(v) = verb {
        memory[2] = v;
    }

    for ip in (0..memory.len()).step_by(4) {
        match memory[ip] {
            1 => {
                let pos1 = memory[ip + 1] as usize;
                let pos2 = memory[ip + 2] as usize;
                let dest = memory[ip + 3] as usize;
                memory[dest] = memory[pos1] + memory[pos2];
            }
            2 => {
                let pos1 = memory[ip + 1] as usize;
                let pos2 = memory[ip + 2] as usize;
                let dest = memory[ip + 3] as usize;
                memory[dest] = memory[pos1] * memory[pos2];
            }
            99 => break,
            _ => break,
        }
    }

    memory[0]
}


impl Day02 {
    pub fn part1(&self, input: &str) -> String {
        let mut memory: Vec<i32> = input.split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        memory[1] = 12;
        memory[2] = 2;

        run_program(&mut memory, None, None);

        memory[0].to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let initial_memory: Vec<i32> = input.split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut memory = initial_memory.clone();
                let output = run_program(&mut memory, Some(noun), Some(verb));

                if output == TARGET {
                    return (100 * noun + verb).to_string(); 
                }
            }
        }
        "Not found".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,13,23,1,23,10,27,1,13,27,31,2,31,10,35,1,35,9,39,1,39,13,43,1,13,43,47,1,47,13,51,1,13,51,55,1,5,55,59,2,10,59,63,1,9,63,67,1,6,67,71,2,71,13,75,2,75,13,79,1,79,9,83,2,83,10,87,1,9,87,91,1,6,91,95,1,95,10,99,1,99,13,103,1,13,103,107,2,13,107,111,1,111,9,115,2,115,10,119,1,119,5,123,1,123,2,127,1,127,5,0,99,2,14,0,0";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT), "4330636");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT), "6086");
    }
}

