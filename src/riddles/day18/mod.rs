use crate::riddles::utils::{Point, Utils};
use crate::riddles::Riddle;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub struct Day18();

impl Riddle for Day18 {
    fn day(&self) -> u8 {
        18
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt", 7, 12), 22)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt", 71, 1024).to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt", 7), String::from("6,1"))
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt", 71)
    }
}

impl Day18 {
    fn _solve_first(&self, filename: &str, size: u8, take: usize) -> u16 {
        let bytes = self.read_input(filename);
        let byte_set: HashSet<&Point<u8>> = HashSet::from_iter(bytes.iter().take(take));
        Self::find_path(&byte_set, size)
    }

    fn _solve_second(&self, filename: &str, size: u8) -> String {
        let bytes = self.read_input(filename);

        for i in (bytes.len() >> 1)..bytes.len() {
            let byte_set: HashSet<&Point<u8>> = HashSet::from_iter(bytes.iter().take(i));
            let res = Self::find_path(&byte_set, size);
            if res == 0xFFFF {
                return format!("{},{}", bytes[i - 1].x, bytes[i - 1].y);
            }
        }

        String::from("Invalid")
    }

    fn read_input(&self, filename: &str) -> Vec<Point<u8>> {
        self.read_input_file(filename)
            .split('\n')
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect()
    }

    fn find_path(bytes: &HashSet<&Point<u8>>, size: u8) -> u16 {
        let mut heap: BinaryHeap<Step> = BinaryHeap::new();
        heap.push(Step {
            distance: 0,
            position: Point { x: 0, y: 0 },
        });

        let target: Point<u8> = Point {
            x: size - 1,
            y: size - 1,
        };

        // let mut visited: HashMap<Point<u8>, u16> = HashMap::new();
        let mut visited: HashSet<Point<u8>> = HashSet::new();

        while let Some(step) = heap.pop() {
            if step.position.x >= size || step.position.y >= size || bytes.contains(&step.position)
            {
                continue;
            }

            if step.position == target {
                return step.distance;
            }

            if !visited.insert(step.position) {
                continue;
            }

            heap.push(step.move_top());
            heap.push(step.move_right());
            heap.push(step.move_bottom());
            heap.push(step.move_left());
        }

        0xFFFF
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Step {
    pub distance: u16,
    pub position: Point<u8>,
}

impl Step {
    fn move_top(&self) -> Step {
        Step {
            distance: self.distance + 1,
            position: Point {
                x: self.position.x,
                y: Self::decrease(self.position.y),
            },
        }
    }

    fn move_right(&self) -> Step {
        Step {
            distance: self.distance + 1,
            position: Point {
                x: self.position.x + 1,
                y: self.position.y,
            },
        }
    }

    fn move_bottom(&self) -> Step {
        Step {
            distance: self.distance + 1,
            position: Point {
                x: self.position.x,
                y: self.position.y + 1,
            },
        }
    }
    fn move_left(&self) -> Step {
        Step {
            distance: self.distance + 1,
            position: Point {
                x: Self::decrease(self.position.x),
                y: self.position.y,
            },
        }
    }

    fn decrease(d: u8) -> u8 {
        if d == 0 {
            0xFF
        } else {
            d - 1
        }
    }
}

impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
