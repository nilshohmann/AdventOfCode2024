use crate::riddles::utils::{ListParsing, Utils};
use crate::riddles::Riddle;
use std::collections::HashMap;

pub struct Day22();

impl Riddle for Day22 {
    fn day(&self) -> u8 {
        22
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test_1.txt"), 37327623)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test_2.txt"), 23)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day22 {
    fn _solve_first(&self, filename: &str) -> u64 {
        let mut result = 0;

        for mut secret in self.read_input_file(filename).split('\n').parse_as::<u64>() {
            for _ in 0..2000 {
                secret = Self::next(secret);
            }

            result += secret;
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> u32 {
        let initial_secrets = self.read_input_file(filename).split('\n').parse_as::<u64>();
        let mut all_prizes: Vec<HashMap<u32, u8>> = Vec::with_capacity(initial_secrets.len());

        for mut secret in initial_secrets {
            let mut prev = (secret % 10) as u8;
            let mut sequence = 0u32;

            let mut best_prizes: HashMap<u32, u8> = HashMap::new();

            for i in 0..2000 {
                secret = Self::next(secret);
                let c = (secret % 10) as u8;

                sequence = (sequence << 8) | (c - prev) as u32;

                if i >= 3 && !best_prizes.contains_key(&sequence) {
                    best_prizes.insert(sequence, c);
                }

                prev = c;
            }

            all_prizes.push(best_prizes);
        }

        let mut best_bananas = 0u32;
        for i in 0..all_prizes.len() {
            let keys = all_prizes[i].keys().cloned().collect::<Vec<u32>>();

            for key in keys {
                let mut count = all_prizes[i][&key] as u32;

                for j in (i + 1)..all_prizes.len() {
                    if let Some(c) = all_prizes[j].remove(&key) {
                        count += c as u32;
                    }
                }

                if count > best_bananas {
                    best_bananas = count;
                }
            }

            all_prizes[i].clear();
        }

        best_bananas
    }

    fn next(secret_number: u64) -> u64 {
        let mut r = (secret_number ^ (secret_number << 6)) % 16777216;
        r = (r ^ (r >> 5)) % 16777216;
        (r ^ (r * 2048)) % 16777216
    }
}
