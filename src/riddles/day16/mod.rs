use crate::riddles::utils::{Point, Utils};
use crate::riddles::Riddle;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Day16();

impl Riddle for Day16 {
    fn day(&self) -> u8 {
        16
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 7036)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 45)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day16 {
    fn _solve_first(&self, filename: &str) -> usize {
        let map = self.read_map(filename);
        let mut visited: HashMap<Point<usize>, usize> = HashMap::new();

        let mut heap: BinaryHeap<Path> = BinaryHeap::new();
        heap.push(Path {
            score: 0,
            position: Self::find(&map, 'S'),
            direction: 1,
            prev: Vec::new(),
        });

        while let Some(path) = heap.pop() {
            if map[path.position.y][path.position.x] == '#' {
                continue;
            }

            if let Some(prev) = visited.get(&path.position) {
                if prev <= &path.score {
                    continue;
                }
            }
            visited.insert(path.position.clone(), path.score);

            if map[path.position.y][path.position.x] == 'E' {
                return path.score;
            }

            heap.push(path.rotate_right());
            heap.push(path.rotate_left());
            heap.push(path.forward());
        }

        0
    }

    fn _solve_second(&self, filename: &str) -> usize {
        let map = self.read_map(filename);
        let mut seats: HashSet<Point<usize>> = HashSet::new();

        let mut heap: BinaryHeap<Path> = BinaryHeap::new();
        heap.push(Path {
            score: 0,
            position: Self::find(&map, 'S'),
            direction: 1,
            prev: Vec::new(),
        });

        let mut visited: HashMap<(Point<usize>, u8), usize> = HashMap::new();
        let mut best_score = 0xFFFFFFFFusize;

        while let Some(path) = heap.pop() {
            if map[path.position.y][path.position.x] == '#' || path.score > best_score {
                continue;
            }

            if let Some(score) = visited.get(&(path.position, path.direction)) {
                if path.score > *score {
                    continue;
                }
            }
            visited.insert((path.position.clone(), path.direction), path.score);

            if map[path.position.y][path.position.x] == 'E' {
                best_score = path.score;

                seats.insert(path.position);
                for p in path.prev {
                    seats.insert(p);
                }

                continue;
            }

            heap.push(path.rotate_right());
            heap.push(path.rotate_left());
            heap.push(path.forward());
        }

        seats.len()
    }

    fn find(map: &Vec<Vec<char>>, c: char) -> Point<usize> {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == c {
                    return Point { x, y };
                }
            }
        }

        panic!("Not found in map ({})", c);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Path {
    pub score: usize,
    pub direction: u8, // top = 0, right = 1, bottom = 2, left = 3
    pub position: Point<usize>,
    pub prev: Vec<Point<usize>>,
}

impl Path {
    pub fn rotate_left(&self) -> Path {
        let direction = (self.direction + 3) % 4;
        Path {
            score: self.score + 1000 + 1,
            position: self.position.forward(direction),
            direction,
            prev: self.extend_prev(),
        }
    }

    pub fn rotate_right(&self) -> Path {
        let direction = (self.direction + 1) % 4;
        Path {
            score: self.score + 1001,
            position: self.position.forward(direction),
            direction,
            prev: self.extend_prev(),
        }
    }

    pub fn forward(&self) -> Path {
        Path {
            score: self.score + 1,
            position: self.position.forward(self.direction),
            direction: self.direction,
            prev: self.extend_prev(),
        }
    }

    fn extend_prev(&self) -> Vec<Point<usize>> {
        self.prev
            .clone()
            .into_iter()
            .chain(vec![self.position].into_iter())
            .collect()
    }
}

impl PartialOrd<Self> for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl Point<usize> {
    pub fn forward(&self, direction: u8) -> Point<usize> {
        match direction {
            0 => self.top(),
            1 => self.right(),
            2 => self.bottom(),
            3 => self.left(),
            _ => panic!("Invalid direction: {}", direction),
        }
    }
}
