use std::collections::HashMap;
use crate::riddles::Riddle;
use crate::riddles::utils::{HashMapExt, Parsing, Utils};

pub struct Day11();

impl Riddle for Day11 {
    fn day(&self) -> u8 { 11 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 55312)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 65601038650482)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day11 {
    fn _solve_first(&self, filename: &str) -> u64 {
        let mut stones = self.read_stones(filename);

        for _ in 0..25 {
            stones = Self::blink(&stones);
        }

        stones.values().sum()
    }

    fn _solve_second(&self, filename: &str) -> u64 {
        let mut stones = self.read_stones(filename);

        for _ in 0..75 {
            stones = Self::blink(&stones);
        }

        stones.values().sum()
    }

    fn read_stones(&self, filename: &str) -> HashMap<u64, u64> {
        let mut stone_map: HashMap<u64, u64> = HashMap::new();

        for value in self.read_input_file(filename).split_whitespace() {
            let number = value.to::<u64>();
            stone_map.add(1, number);
        }

        stone_map
    }

    fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (number, count) in stones.iter() {
            if *number == 0 {
                new_stones.add(*count, 1);
                continue;
            }

            let digits = digits(*number);
            if digits & 1 == 1 {
                new_stones.add(*count, number * 2024);
            } else {
                let d = 10u64.pow(digits >> 1);
                new_stones.add(*count, number / d);
                new_stones.add(*count, number % d);
            }
        }

        new_stones
    }
}

fn digits(number: u64) -> u32 {
    let mut digits = 0;
    let mut number = number;

    while number > 0 {
        number /= 10;
        digits += 1;
    }

    digits
}
