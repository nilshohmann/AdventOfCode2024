use crate::riddles::utils::{HashMapExt, Utils};
use crate::riddles::Riddle;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day23();

impl Riddle for Day23 {
    fn day(&self) -> u8 {
        23
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 7)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(
            self._solve_second("input_test.txt"),
            String::from("co,de,ka,ta"),
        )
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt")
    }
}

impl Day23 {
    fn _solve_first(&self, filename: &str) -> usize {
        let connections = self.read_connections(filename);

        let mut interconnected: HashSet<String> = HashSet::new();
        for (first, next) in connections.iter().filter(|(k, _)| k.starts_with("t")) {
            for second in next.iter() {
                for third in connections[second].iter() {
                    if third == first {
                        continue;
                    }

                    if connections[third].contains(first) {
                        let mut group = [first.clone(), second.clone(), third.clone()];
                        group.sort();
                        interconnected.insert(group.join(","));
                    }
                }
            }
        }

        interconnected.len()
    }

    fn _solve_second(&self, filename: &str) -> String {
        let connections = self.read_connections(filename);

        let keys: Vec<String> = connections.keys().cloned().collect();

        let mut longest: Vec<String> = Vec::new();

        let mut states: VecDeque<(usize, Vec<&String>)> = VecDeque::new();
        for i in 0..keys.len() - 2 {
            states.push_back((i, Vec::new()));
        }

        while let Some((index, path)) = states.pop_front() {
            let c = keys.get(index).unwrap();

            let next = connections.get(c).unwrap();
            if path.len() > 1 && !path.iter().all(|p| next.contains(p)) {
                continue;
            }

            let path: Vec<&String> = path.into_iter().chain([c].into_iter()).collect();
            if path.len() > longest.len() {
                longest = path.iter().map(|n| String::from(*n)).collect();
            }

            for j in (index + 1)..keys.len() {
                let n = &keys[j];
                if next.contains(n) && !path.contains(&n) {
                    states.push_back((j, path.clone()))
                }
            }
        }

        longest.sort();
        longest.join(",")
    }

    fn read_connections(&self, filename: &str) -> HashMap<String, Vec<String>> {
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        for line in self.read_input_file(filename).split('\n') {
            let (left, right) = line.split_once('-').unwrap();

            connections.add(String::from(left), String::from(right));
            connections.add(String::from(right), String::from(left));
        }

        connections
    }
}

impl HashMapExt<String, String> for HashMap<String, Vec<String>> {
    fn add(&mut self, value: String, key: String) {
        let mut list = vec![value];
        if let Some(prev) = self.remove(&key) {
            list = prev.into_iter().chain(list.into_iter()).collect();
        }

        self.insert(key, list);
    }
}
