use crate::riddles::{ListParsing, Parsing, Riddle, Utils};

pub struct Day05();

impl Riddle for Day05 {
    fn day(&self) -> u8 { 5 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 143)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 123)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day05 {
    fn _solve_first(&self, filename: &str) -> i32 {
        let (rules, updates) = self.read_rules_and_updates(filename);

        // > 106
        let mut result = 0;
        for update in updates {
            if self.is_valid(&update, &rules) {
                result += update[update.len() / 2];
            }
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> i32 {
        let (rules, updates) = self.read_rules_and_updates(filename);

        // > 106
        let mut result = 0;
        for update in updates {
            let mut update = update;
            if self.reorder_update(&mut update, &rules) {
                result += update[update.len() / 2];
            }
        }

        result
    }

    fn read_rules_and_updates(&self, filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let data = self.read_input_file(filename);
        let (raw_rules, raw_updates) = data.split_once("\n\n").unwrap();

        let rules = raw_rules.split("\n")
            .map(|line| line.split_once("|").map(|(l, r)| (l.to::<i32>(), r.to::<i32>())).unwrap())
            .collect::<Vec<(i32, i32)>>();

        let updates = raw_updates.split("\n")
            .map(|line| line.split(",").parse_as::<i32>())
            .collect::<Vec<Vec<i32>>>();

        (rules, updates)
    }

    fn is_valid(&self, update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
        for (l, r) in rules {
            let l_index = update.iter().position(|e| e == l);
            let r_index = update.iter().position(|e| e == r);

            if l_index.is_some() && r_index.is_some() && r_index.unwrap() < l_index.unwrap() {
                return false;
            }
        }

        true
    }

    fn reorder_update(&self, update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
        let mut reordered = false;

        for (l, r) in rules {
            let l_index = update.iter().position(|e| e == l);
            let r_index = update.iter().position(|e| e == r);

            if l_index.is_some() && r_index.is_some() && r_index.unwrap() < l_index.unwrap() {
                let t = update[l_index.unwrap()];
                update[l_index.unwrap()] = update[r_index.unwrap()];
                update[r_index.unwrap()] = t;

                reordered = true;
            }
        }

        if !reordered {
            return false;
        }

        while reordered {
            reordered = self.reorder_update(update, rules);
        }

        true
    }
}
