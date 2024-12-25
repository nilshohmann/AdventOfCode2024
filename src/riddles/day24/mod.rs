use crate::riddles::utils::Utils;
use crate::riddles::Riddle;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}

struct Connection {
    pub left: String,
    pub right: String,
    pub operation: Operation,
    pub target: String,
}

pub struct Day24();

impl Riddle for Day24 {
    fn day(&self) -> u8 {
        24
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 2024)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        true
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt")
    }
}

impl Day24 {
    fn _solve_first(&self, filename: &str) -> u64 {
        let (mut values, mut connections) = self.read_input(filename);

        Self::solve(&mut values, &mut connections)
    }

    fn _solve_second(&self, filename: &str) -> String {
        let (_, mut connections) = self.read_input(filename);

        // Found by manual search below
        let swap_pairs = [
            ("vss", "z14"),
            ("hjf", "kdh"),
            ("kpp", "z31"),
            ("sgj", "z35"),
        ];

        let mut swaps: Vec<String> = Vec::with_capacity(8);
        for (first, second) in swap_pairs {
            swaps.push(String::from(first));
            swaps.push(String::from(second));

            connections.swap_target(first, second);
        }

        // Each next Z contains the left part of the XOR from the previous one plus the same
        // pattern of next checks
        // z00 -> y00 XOR x00
        // z01 -> (y00 AND x00) XOR (x01 XOR y01)
        // z02 -> (((y00 AND x00) AND (x01 XOR y01)) OR (y01 AND x01)) XOR (x02 XOR y02)
        // z03 -> (((((y00 AND x00) AND (x01 XOR y01)) OR (y01 AND x01)) AND (x02 XOR y02)) OR (y02 AND x02)) XOR (y03 XOR x03)

        // So we go through all Zs and reconstruct the path to find errors / swapped targets
        let mut connection_map: HashMap<(&String, Operation, &String), &String> = HashMap::new();
        for c in connections.iter() {
            connection_map.insert((&c.left, c.operation, &c.right), &c.target);
            connection_map.insert((&c.right, c.operation, &c.left), &c.target);
        }

        let get = |left: &String, op: Operation, right: &String| {
            if let Some(res) = connection_map.get(&(left, op, right)) {
                return res.to_owned();
            }

            panic!("Failed to get {} {:?} {}", left, op, right);
        };

        let x = (0u64..46)
            .map(|i| format!("x{:0>2}", i))
            .collect::<Vec<String>>();
        let y = (0u64..46)
            .map(|i| format!("y{:0>2}", i))
            .collect::<Vec<String>>();

        let verify = |i: usize, l: &String| {
            let r = get(&x[i], Operation::Xor, &y[i]);

            let z = get(l, Operation::Xor, r);
            let expected = format!("z{:0>2}", i);
            if z != &expected {
                panic!("Invalid result for {}: {}", &expected, z);
            }
        };

        // Left part that's adjusted every step in Z
        let mut l = get(&x[0], Operation::And, &y[0]);
        verify(1, &l);

        for i in 2..45 {
            let t2 = get(&x[i - 1], Operation::Xor, &y[i - 1]);
            let ll2 = get(l, Operation::And, t2);
            let lr2 = get(&x[i - 1], Operation::And, &y[i - 1]);

            l = get(ll2, Operation::Or, lr2);
            verify(i, &l);
        }

        swaps.sort();
        swaps.join(",")
    }

    fn solve(values: &mut HashMap<String, bool>, connections: &mut VecDeque<Connection>) -> u64 {
        while let Some(connection) = connections.pop_front() {
            if !values.contains_key(&connection.left) || !values.contains_key(&connection.right) {
                connections.push_back(connection);
                continue;
            }

            let left = values.get(&connection.left).unwrap();
            let right = values.get(&connection.right).unwrap();

            let v = match connection.operation {
                Operation::And => *left && *right,
                Operation::Or => *left || *right,
                Operation::Xor => left != right,
            };
            values.insert(connection.target, v);
        }

        let mut result = 0u64;

        let mut z = 0u8;
        while let Some(value) = values.get(&format!("z{:0>2}", z)) {
            result = result | ((*value as u64) << (z as u64));
            z += 1;
        }

        result
    }

    fn read_input(&self, filename: &str) -> (HashMap<String, bool>, VecDeque<Connection>) {
        let data = self.read_input_file(filename);
        let (top, bottom) = data.split_once("\n\n").unwrap();
        let mut values: HashMap<String, bool> = HashMap::new();
        let mut connections: VecDeque<Connection> = VecDeque::new();

        for line in top.split('\n') {
            let (var, val) = line.split_once(": ").unwrap();
            values.insert(var.to_string(), val == "1");
        }

        for line in bottom.split('\n') {
            let (left, var) = line.split_once(" -> ").unwrap();
            let left = left.split(' ').collect::<Vec<&str>>();
            let operation = match left[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("Unknown operation: {}", left[1]),
            };

            connections.push_back(Connection {
                left: left[0].to_string(),
                right: left[2].to_string(),
                target: var.to_string(),
                operation,
            });
        }

        (values, connections)
    }
}

trait Swapping {
    fn swap_target(&mut self, first: &str, second: &str);
}

impl Swapping for VecDeque<Connection> {
    fn swap_target(&mut self, first: &str, second: &str) {
        self.iter_mut().for_each(|c| {
            if c.target == first {
                c.target = String::from(second);
            } else if c.target == second {
                c.target = String::from(first);
            }
        });
    }
}
