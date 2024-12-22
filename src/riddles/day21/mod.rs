use crate::riddles::utils::{Parsing, Point, Utils};
use crate::riddles::Riddle;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day21();

impl Riddle for Day21 {
    fn day(&self) -> u8 {
        21
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve("input_test.txt", 2), 126384)
    }

    fn solve_first(&self) -> String {
        self._solve("input.txt", 2).to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve("input_test.txt", 25), 154115708116294)
    }

    fn solve_second(&self) -> String {
        self._solve("input.txt", 25).to_string()
    }
}

impl Day21 {
    fn _solve(&self, filename: &str, robots: u8) -> u64 {
        let data = self.read_input_file(filename);
        let mut possible_moves: HashMap<(char, char), HashSet<String>> = HashMap::new();
        Self::add_possible_moves("789\n456\n123\n 0A", &mut possible_moves);
        Self::add_possible_moves(" ^A\n<v>", &mut possible_moves);

        let mut result = 0;
        let mut cache = HashMap::<(String, u8), u64>::new();

        for code in data.split('\n').map(|s| String::from(s)) {
            let length = Self::find_best_sequence(&code, robots, &possible_moves, &mut cache);
            let num = code.replace('A', "").to::<u64>();

            result += num * length;
        }

        result
    }

    fn find_best_sequence(
        data: &String,
        pending: u8,
        possible_moves: &HashMap<(char, char), HashSet<String>>,
        cache: &mut HashMap<(String, u8), u64>,
    ) -> u64 {
        if let Some(result) = cache.get(&(data.clone(), pending)) {
            return *result;
        }

        let mut result = 0u64;

        // All direction keypad inputs start and end with an 'A'
        let mut prev = 'A';
        for c in data.chars() {
            let moves = possible_moves.get(&(prev, c)).unwrap();
            if pending == 0 {
                result += moves.iter().next().unwrap().len() as u64;
            } else {
                result += moves
                    .iter()
                    .map(|m| Self::find_best_sequence(m, pending - 1, possible_moves, cache))
                    .min()
                    .unwrap();
            }

            prev = c;
        }

        cache.insert((data.clone(), pending), result);
        result
    }

    fn add_possible_moves(keys: &str, map: &mut HashMap<(char, char), HashSet<String>>) {
        let keypad = keys
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        for y in 0..keypad.len() {
            for x in 0..keypad[0].len() {
                let c = keypad[y][x];
                if c == ' ' {
                    continue;
                }

                let mut moves = HashMap::<char, Vec<String>>::new();

                let mut togo = VecDeque::<(Point<usize>, Vec<char>)>::with_capacity(20);
                togo.push_back((Point { x, y }, Vec::<char>::new()));

                while let Some((p, path)) = togo.pop_front() {
                    let t = keypad[p.y][p.x];

                    if t == ' ' {
                        continue;
                    }

                    if moves.contains_key(&t) {
                        let m = moves.get_mut(&t).unwrap();
                        if m[0].len() < path.len() {
                            // We already found one or more better paths
                            continue;
                        }
                        m.push(path.iter().chain(['A'].iter()).collect());
                    } else {
                        moves.insert(t, vec![path.iter().chain(['A'].iter()).collect()]);
                    }

                    if p.y > 0 {
                        togo.push_back((p.top(), [path.clone(), vec!['^']].concat()));
                    }
                    if p.y < keypad.len() - 1 {
                        togo.push_back((p.bottom(), [path.clone(), vec!['v']].concat()));
                    }
                    if p.x > 0 {
                        togo.push_back((p.left(), [path.clone(), vec!['<']].concat()));
                    }
                    if p.x < keypad[0].len() - 1 {
                        togo.push_back((p.right(), [path.clone(), vec!['>']].concat()));
                    }
                }

                for (k, v) in moves {
                    map.insert((c, k), HashSet::from_iter(v.into_iter()));
                }
            }
        }
    }
}
