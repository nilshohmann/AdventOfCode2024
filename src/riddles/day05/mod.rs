use crate::riddles::Riddle;

pub struct Day05();

impl Riddle for Day05 {
    fn day(&self) -> u8 { 5 }

    fn solve_first(&self) -> String {
        let (rules, updates) = self.read_rules_and_updates();

        // > 106
        let mut result = 0;
        for update in updates {
            if self.is_valid(&update, &rules) {
                result += update[update.len() / 2];
            }
        }
        result.to_string()
    }

    fn solve_second(&self) -> String {
        let (rules, updates) = self.read_rules_and_updates();

        // > 106
        let mut result = 0;
        for update in updates {
            let mut update = update;
            if self.reorder_update(&mut update, &rules) {
                result += update[update.len() / 2];
            }
        }
        result.to_string()
    }
}

impl Day05 {
    fn read_rules_and_updates(&self) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let data = self.read_input_file("input.txt");
        let (raw_rules, raw_updates) = data.split_once("\n\n").unwrap();

        let rules = raw_rules.split("\n")
            .map(|line| line.split_once("|").map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())).unwrap())
            .collect::<Vec<(i32, i32)>>();

        let updates = raw_updates.split("\n")
            .map(|line| line.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>())
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