use crate::riddles::utils::Utils;
use crate::riddles::Riddle;
use std::iter::zip;

pub struct Day25();

impl Riddle for Day25 {
    fn day(&self) -> u8 {
        25
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 3)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        true
    }

    fn solve_second(&self) -> String {
        String::from("the end")
    }
}

impl Day25 {
    fn _solve_first(&self, filename: &str) -> usize {
        let (locks, keys) = self.read_locks_and_keys(filename);

        let mut result = 0;

        for lock in locks.iter() {
            for key in keys.iter() {
                if zip(lock, key).all(|(a, b)| a + b <= 5) {
                    result += 1;
                }
            }
        }

        result
    }

    fn read_locks_and_keys(&self, filename: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
        let mut locks: Vec<[u8; 5]> = Vec::new();
        let mut keys: Vec<[u8; 5]> = Vec::new();

        let all_data = self.read_input_file(filename);
        for data in all_data.split("\n\n") {
            let mut values = [0u8; 5];

            for line in data.split('\n').skip(1).take(5) {
                for (i, c) in line.chars().enumerate() {
                    values[i] += if c == '#' { 1 } else { 0 };
                }
            }

            if data.chars().next() == Some('#') {
                locks.push(values);
            } else {
                keys.push(values);
            }
        }

        (locks, keys)
    }
}
