use regex::Regex;
use crate::riddles::Riddle;

pub struct Day03();

impl Riddle for Day03 {
    fn day(&self) -> u8 { 3 }

    fn solve_first(&self) -> String {
        let mut result = 0;
        let input = self.read_input_file("input.txt");

        let re = Regex::new(r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)").unwrap();
        for (_, [first, second]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
            result += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        }

        result.to_string()
    }

    fn solve_second(&self) -> String {
        let mut result = 0;
        let input = self.read_input_file("input.txt");

        let mut is_enabled = true;

        let re = Regex::new(r"((mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\))|(do\(\))|(don't\(\)))").unwrap();
        for m in re.find_iter(input.as_str()) { //.map(|c| c.extract()) {
            let s = m.as_str();
            if s == "don't()" {
                is_enabled = false;
            } else if s == "do()" {
                is_enabled = true;
            } else if is_enabled {
                let values = s[4..s.len()-1].split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                result += values[0] * values[1];
            }
        }

        result.to_string()
    }
}
