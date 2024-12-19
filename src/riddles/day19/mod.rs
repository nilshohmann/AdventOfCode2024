use crate::riddles::utils::Utils;
use crate::riddles::Riddle;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::iter::zip;
use std::ops::AddAssign;

pub struct Day19();

impl Riddle for Day19 {
    fn day(&self) -> u8 {
        19
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self.solve("input_test.txt", true), 6)
    }

    fn solve_first(&self) -> String {
        self.solve("input.txt", true).to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self.solve("input_test.txt", false), 16)
    }

    fn solve_second(&self) -> String {
        self.solve("input.txt", false).to_string()
    }
}

impl Day19 {
    fn solve(&self, filename: &str, return_when_found: bool) -> u64 {
        let data = self.read_input_file(filename);
        let (top, bottom) = data.split_once("\n\n").unwrap();

        let towels = top.split(", ").collect::<Vec<&str>>();
        let patterns = bottom.split("\n").collect::<Vec<&str>>();

        patterns
            .iter()
            .map(|pattern| Self::find_matching_towels(pattern, &towels, return_when_found))
            .sum()
    }

    fn find_matching_towels(pattern: &str, towels: &Vec<&str>, return_when_found: bool) -> u64 {
        let mut counts = HashMap::<usize, u64>::new();
        counts.insert(0, 1);

        let mut states = BinaryHeap::<Reverse<usize>>::new();
        states.push(Reverse(0));

        while let Some(offset) = states.pop() {
            let offset = offset.0;

            if offset == pattern.len() {
                if return_when_found {
                    return 1;
                }
                continue;
            }

            for towel in towels.iter() {
                let next_offset = offset + towel.len();
                if next_offset > pattern.len() {
                    continue;
                }

                if zip(pattern.chars().skip(offset), towel.chars()).all(|(a, b)| a == b) {
                    let prev = counts[&offset];
                    if let Some(count) = counts.get_mut(&next_offset) {
                        count.add_assign(prev);
                        continue;
                    }

                    counts.insert(next_offset, prev);
                    states.push(Reverse(next_offset));
                }
            }
        }

        *counts.get(&pattern.len()).unwrap_or(&0)
    }
}
