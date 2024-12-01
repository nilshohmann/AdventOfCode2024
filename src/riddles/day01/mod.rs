use std::collections::HashMap;
use std::iter::zip;
use crate::riddles::Riddle;

pub struct Day01();

impl Riddle for Day01 {
    fn day(&self) -> u8 { 1 }

    fn solve_first(&self) -> String {
        let (left_list, right_list) = self.read_lists();

        let mut result: i32 = 0;
        for (left, right) in zip(left_list, right_list) {
            result += (right - left).abs();
        }

        result.to_string()
    }

    fn solve_second(&self) -> String {
        let (left_list, right_list) = self.read_lists();

        let mut mapping = HashMap::new();
        for right in right_list {
            mapping.insert(right, if mapping.contains_key(&right) {
                mapping[&right] + 1
            } else {
                1
            });
        }

        let mut result: i32 = 0;
        for left in left_list {
            if mapping.contains_key(&left) {
                result += left * mapping[&left]
            }
        }

        result.to_string()
    }
}

impl Day01 {
    fn read_lists(&self) -> (Vec<i32>, Vec<i32>) {
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();

        for line in self.read_input_file("input.txt").split("\n") {
            let data = line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            left_list.push(data[0]);
            right_list.push(data[1]);
        }

        left_list.sort();
        right_list.sort();
        (left_list, right_list)
    }
}
