use std::collections::HashMap;
use std::iter::zip;
use crate::riddles::{HashMapExt, ListParsing, Riddle, Utils};

pub struct Day01();

impl Riddle for Day01 {
    fn day(&self) -> u8 { 1 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 11)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 31)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day01 {
    fn _solve_first(&self, filename: &str) -> i32 {
        let (left_list, right_list) = self.read_lists(filename);

        let mut result: i32 = 0;
        for (left, right) in zip(left_list, right_list) {
            result += (right - left).abs();
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> i32 {
        let (left_list, right_list) = self.read_lists(filename);

        let mut mapping = HashMap::new();
        for right in right_list {
            mapping.add(1, right);
        }

        let mut result: i32 = 0;
        for left in left_list {
            if mapping.contains_key(&left) {
                result += left * mapping[&left]
            }
        }

        result
    }

    fn read_lists(&self, filename: &str) -> (Vec<i32>, Vec<i32>) {
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();

        for line in self.read_input_file(filename).split("\n") {
            let data = line.split_whitespace().parse_as::<i32>();

            left_list.push(data[0]);
            right_list.push(data[1]);
        }

        left_list.sort();
        right_list.sort();
        (left_list, right_list)
    }
}