use std::cmp::min;
use crate::riddles::Riddle;
use crate::riddles::utils::{Parsing, Point, Utils};

#[derive(Debug)]
struct Button {
    diff: Point<u64>,
    cost: u64,
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    target: Point<u64>,
}

impl Machine {
    fn verify(&self, a: u64, b: u64) -> bool {
        let p = Point {
            x: a * self.a.diff.x + b * self.b.diff.x,
            y: a * self.a.diff.y + b * self.b.diff.y,
        };

        p == self.target
    }
}

pub struct Day13();

impl Riddle for Day13 {
    fn day(&self) -> u8 {
        13
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 480)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 875318608908)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day13 {
    fn _solve_first(&self, filename: &str) -> u64 {
        let machines = self.read_machines(filename, Point { x: 0, y: 0 });

        let mut result = 0u64;
        for machine in machines.iter() {
            let max_a = min(
                machine.target.x / machine.a.diff.x,
                machine.target.y / machine.a.diff.y,
            );

            let mut results: Vec<u64> = Vec::with_capacity(max_a as usize);
            for a in 0..max_a + 1 {
                let r_x = machine.target.x - a * machine.a.diff.x;

                if r_x % machine.b.diff.x == 0 {
                    let b = r_x / machine.b.diff.x;
                    if machine.verify(a, b) {
                        results.push(a * machine.a.cost + b * machine.b.cost);
                    }
                }
            }

            if !results.is_empty() {
                result += *results.iter().min().unwrap();
            }
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> u64 {
        let machines = self.read_machines(
            filename,
            Point {
                x: 10000000000000,
                y: 10000000000000,
            },
        );

        let mut result = 0u64;

        for machine in machines.iter() {
            // a * a.x + b * b.x = t.x
            // a * a.y + b * b.y = t.y
            // => b = (t.x / a.x - t.y / a.y) / (b.x / a.x - b.y / a.y)
            let ba = machine.a.diff.convert(|v| *v as f64);
            let bb = machine.b.diff.convert(|v| *v as f64);
            let t = machine.target.convert(|v| *v as f64);

            let b = ((t.x / ba.x - t.y / ba.y) / (bb.x / ba.x - bb.y / ba.y)).round() as u64;
            let a = ((t.x - (b as f64) * bb.x) / ba.x).round() as u64;

            if machine.verify(a, b) {
                result += a * machine.a.cost + b * machine.b.cost;
            }
        }

        result
    }

    fn read_machines(&self, filename: &str, offset: Point<u64>) -> Vec<Machine> {
        let mut result = Vec::new();
        let read_point = |data: &str, sep: char| {
            let (left, right) = data.split_once(", ").unwrap();

            Point {
                x: left.split_once(sep).unwrap().1.to::<u64>(),
                y: right.split_once(sep).unwrap().1.to::<u64>(),
            }
        };

        for data in self.read_input_file(filename).split("\n\n") {
            let lines = data
                .split('\n')
                .map(|s| s.split_once(": ").unwrap().1)
                .collect::<Vec<&str>>();

            let a = read_point(lines[0], '+');
            let b = read_point(lines[1], '+');
            let target = read_point(lines[2], '=');

            result.push(Machine {
                a: Button { diff: a, cost: 3 },
                b: Button { diff: b, cost: 1 },
                target: Point {
                    x: target.x + offset.x,
                    y: target.y + offset.y,
                },
            });
        }

        result
    }
}
