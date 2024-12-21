use crate::riddles::utils::{Point, Utils};
use crate::riddles::Riddle;
use std::collections::{HashSet, VecDeque};

pub struct Day20();

impl Riddle for Day20 {
    fn day(&self) -> u8 {
        20
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve("input_test.txt", |v| v == 2, 20), 5)
    }

    fn solve_first(&self) -> String {
        self._solve("input.txt", |v| v == 2, 100).to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve("input_test.txt", |v| v <= 20, 50), 285)
    }

    fn solve_second(&self) -> String {
        self._solve("input.txt", |v| v <= 20, 100).to_string()
    }
}

impl Day20 {
    // The maze is constructed in a way that the farthest distance to travel is from start to end.
    // So we don't need to check whether we would actually reach the end but only if we saved
    // enough steps with a cheat.
    fn _solve(&self, filename: &str, check: fn(usize) -> bool, min_save: usize) -> usize {
        let map = self.read_map(filename);

        let distances = Self::find_distances(&map, Self::find(&map, 'S'));

        let mut result = 0;

        for i in 0..distances.len() - 1 {
            let (start, steps1) = distances[i];

            // As the list of distances is sorted by the amount of steps need to reach them we
            // only need to check the items after the current as it doesn't make sense to cheat
            // to a point we already reached without the cheat.
            for j in (i + 1)..distances.len() {
                let (end, steps2) = distances[j];

                // Actual distance between the two points we save by cheating
                let cheat_distance = start.distance(end);
                let saved_steps = steps2 - steps1;

                if check(cheat_distance) && saved_steps - cheat_distance >= min_save {
                    result += 1;
                }
            }
        }

        result
    }

    fn find_distances(map: &Vec<Vec<char>>, start: Point<u8>) -> Vec<(Point<u8>, usize)> {
        let mut result = Vec::<(Point<u8>, usize)>::with_capacity(10_000);

        let mut visited = HashSet::<Point<u8>>::with_capacity(10_000);
        let mut togo = VecDeque::<(Point<u8>, usize)>::with_capacity(100);
        togo.push_back((start, 0));

        while let Some((p, steps)) = togo.pop_front() {
            if map[p.y as usize][p.x as usize] == '#' || !visited.insert(p) {
                continue;
            }

            result.push((p, steps));

            togo.push_back((p.top(), steps + 1));
            togo.push_back((p.right(), steps + 1));
            togo.push_back((p.bottom(), steps + 1));
            togo.push_back((p.left(), steps + 1));
        }

        result
    }

    fn find(map: &Vec<Vec<char>>, c: char) -> Point<u8> {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == c {
                    return Point {
                        x: x as u8,
                        y: y as u8,
                    };
                }
            }
        }

        panic!("Not found in map ({})", c);
    }
}

impl Point<u8> {
    pub fn top(&self) -> Point<u8> {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn right(&self) -> Point<u8> {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn bottom(&self) -> Point<u8> {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Point<u8> {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn distance(&self, to: Self) -> usize {
        let dy = if self.y < to.y {
            to.y - self.y
        } else {
            self.y - to.y
        };
        let dx = if self.x < to.x {
            to.x - self.x
        } else {
            self.x - to.x
        };

        dx as usize + dy as usize
    }
}
