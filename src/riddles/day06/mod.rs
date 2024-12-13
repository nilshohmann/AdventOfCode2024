use std::collections::HashSet;
use crate::riddles::Riddle;
use crate::riddles::utils::{Point, Utils};

pub struct Day06();

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    point: Point<usize>,
    direction: usize,
}

impl Riddle for Day06 {
    fn day(&self) -> u8 { 6 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 41)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 6)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day06 {
    fn _solve_first(&self, filename: &str) -> usize {
        let map = self.read_map(filename);

        let start = self.find_guard(&map);
        let visited = self.find_guard_movement(&map, &start);

        visited.len()
    }

    fn _solve_second(&self, filename: &str) -> i32 {
        let mut map = self.read_map(filename);

        let start = self.find_guard(&map);
        let visited = self.find_guard_movement(&map, &start);

        let mut result = 0;

        // As we can only block the way the guard is moving, we only need to check this path
        for obstacle in visited {
            if map[obstacle.y][obstacle.x] == '.' {
                // Insert a temporary obstacle
                map[obstacle.y][obstacle.x] = '#';

                if self.is_looping(&map, &start) {
                    result += 1;
                }

                // Remove obstacle afterward
                map[obstacle.y][obstacle.x] = '.';
            }
        }

        result
    }

    fn find_guard(&self, map: &Vec<Vec<char>>) -> Position {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '^' {
                    return Position { point: Point { x, y }, direction: UP };
                }
            }
        }

        panic!("Couldn't find guard position!");
    }

    fn find_guard_movement(&self, map: &Vec<Vec<char>>, start: &Position) -> HashSet<Point<usize>> {
        let mut visited: HashSet<Point<usize>> = HashSet::new();

        let mut current = start.clone();
        visited.insert(current.point.clone());

        while self.move_guard(&map, &mut current) {
            visited.insert(current.point.clone());
        }

        visited
    }

    fn move_guard(&self, map: &Vec<Vec<char>>, pos: &mut Position) -> bool {
        let next = self.move_forward(pos);
        if next.y >= map.len() || next.x >= map[next.y].len() {
            // We left the map
            return false;
        }

        if map[next.y][next.x] == '#' {
            // Rotate 90° to the right
            pos.direction = (pos.direction + 1) % 4;
        } else {
            // Move forward
            pos.point = next;
        }

        true
    }

    fn move_forward(&self, pos: &Position) -> Point<usize> {
        let decrease = move |i: usize| if i == 0 { 0xFFFF } else { i - 1 };
        let (x, y) = (pos.point.x, pos.point.y);

        match pos.direction {
            UP => Point { x, y: decrease(y) },
            RIGHT => Point { x: x + 1, y },
            DOWN => Point { x, y: y + 1 },
            LEFT => Point { x: decrease(x), y },
            _ => panic!("Invalid direction: {}!", pos.direction),
        }
    }

    fn is_looping(&self, map: &Vec<Vec<char>>, start: &Position) -> bool {
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(start.clone());

        let mut current = start.clone();
        while self.move_guard(&map, &mut current) {
            if !visited.insert(current.clone()) {
                return true;
            }
        }

        false
    }
}
