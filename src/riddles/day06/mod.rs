use std::collections::HashSet;
use crate::riddles::Riddle;

pub struct Day06();

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

impl Riddle for Day06 {
    fn day(&self) -> u8 { 6 }

    fn solve_first(&self) -> String {
        let map = self.read_map();

        let (y, x) = self.find_guard(&map);
        let visited = self.find_guard_movement(&map, y, x, UP);

        visited.len().to_string()
    }

    fn solve_second(&self) -> String {
        let mut map = self.read_map();

        let (y, x) = self.find_guard(&map);
        let visited = self.find_guard_movement(&map, y, x, UP);

        let mut result = 0;

        // As we can only block the way the guard is moving, we only need to check this path
        for (o_y, o_x) in visited {
            if map[o_y][o_x] == '.' {
                // Insert a temporary obstacle
                map[o_y][o_x] = '#';

                if self.is_looping(&map, y, x, 0) {
                    result += 1;
                }

                // Remove obstacle afterward
                map[o_y][o_x] = '.';
            }
        }

        result.to_string()
    }
}

impl Day06 {
    fn read_map(&self) -> Vec<Vec<char>> {
        self.read_input_file("input.txt")
            .split("\n")
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn find_guard(&self, map: &Vec<Vec<char>>) -> (usize, usize) {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '^' {
                    return (y, x);
                }
            }
        }

        panic!("Couldn't find guard position!");
    }

    fn find_guard_movement(&self, map: &Vec<Vec<char>>, y: usize, x: usize, d: usize) -> HashSet<(usize, usize)> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((y, x));

        let (mut x, mut y, mut d) = (x, y, d);
        while self.move_guard(&map, &mut y, &mut x, &mut d) {
            visited.insert((y, x));
        }

        visited
    }

    fn move_guard(&self, map: &Vec<Vec<char>>, y: &mut usize, x: &mut usize, d: &mut usize) -> bool {
        let (mut next_y, mut next_x) = self.move_forward(*y, *x, *d);
        if next_y >= map.len() || next_x >= map[next_y].len() {
            return false;
        }

        if map[next_y][next_x] == '#' {
            *d = (*d + 1) % 4;
            (next_y, next_x) = self.move_forward(*y, *x, *d);
            if next_y >= map.len() || next_x >= map[next_y].len() {
                return false;
            }

            // We got stuck in a corner, go back 180°
            if map[next_y][next_x] == '#' {
                *d = (*d + 1) % 4;
                return true;
            }
        }

        // Update current position
        *y = next_y;
        *x = next_x;
        true
    }

    fn move_forward(&self, y: usize, x: usize, d: usize) -> (usize, usize) {
        match d {
            UP => (if y == 0 { 0xFFFF } else { y - 1 }, x),
            RIGHT => (y, x + 1),
            DOWN => (y + 1, x),
            LEFT => (y, if x == 0 { 0xFFFF } else { x - 1 }),
            _ => panic!("Invalid direction: {}!", d),
        }
    }

    fn is_looping(&self, map: &Vec<Vec<char>>, y: usize, x: usize, d: usize) -> bool {
        let (mut y, mut x, mut d) = (y, x, d);

        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
        visited.insert((y, x, d));

        while self.move_guard(&map, &mut y, &mut x, &mut d) {
            if !visited.insert((y, x, d)) {
                return true;
            }
        }

        false
    }
}
