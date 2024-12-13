use regex::Regex;
use crate::riddles::Riddle;
use crate::riddles::utils::{ListParsing, Parsing, Utils};

pub struct Day03();

impl Riddle for Day03 {
    fn day(&self) -> u8 { 3 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 161)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 48)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day03 {
    fn _solve_first(&self, filename: &str) -> i32 {
        let mut result = 0;
        let input = self.read_input_file(filename);

        let re = Regex::new(r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)").unwrap();
        for (_, [first, second]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
            result += first.to::<i32>() * second.to::<i32>();
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> i32 {
        let mut result = 0;
        let input = self.read_input_file(filename);

        let mut is_enabled = true;

        let re = Regex::new(r"((mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\))|(do\(\))|(don't\(\)))").unwrap();
        for m in re.find_iter(input.as_str()) {
            let s = m.as_str();
            if s == "don't()" {
                is_enabled = false;
            } else if s == "do()" {
                is_enabled = true;
            } else if is_enabled {
                let values = s[4..s.len() - 1].split(",").parse_as::<i32>();
                result += values[0] * values[1];
            }
        }

        result
    }
}
