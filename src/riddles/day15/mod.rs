use crate::riddles::utils::{Point, Utils};
use crate::riddles::Riddle;

impl Point<usize> {
    fn next(&self, m: char) -> Point<usize> {
        match m {
            '^' => self.top(),
            '>' => self.right(),
            'v' => self.bottom(),
            '<' => self.left(),
            _ => panic!("Invalid move: {}", m),
        }
    }
}

pub struct Day15();

impl Riddle for Day15 {
    fn day(&self) -> u8 {
        15
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 10092)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 9021)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day15 {
    fn _solve_first(&self, filename: &str) -> usize {
        let (mut map, moves) = self.read_input(filename);

        let mut robot = Self::find_robot(&map);
        map[robot.y][robot.x] = '.';

        for m in moves.chars() {
            let next = robot.next(m);

            if map[next.y][next.x] == '.' {
                robot = next;
            } else if map[next.y][next.x] == 'O' {
                let mut next_box = next.next(m);
                while map[next_box.y][next_box.x] == 'O' {
                    next_box = next_box.next(m);
                }

                if map[next_box.y][next_box.x] == '.' {
                    map[next_box.y][next_box.x] = 'O';
                    map[next.y][next.x] = '.';
                    robot = next;
                }
            }
        }

        Self::get_gps_positions(&map, 'O')
    }

    fn _solve_second(&self, filename: &str) -> usize {
        let (mut map, moves) = self.read_input(filename);
        map = self.expand_map(&mut map);

        let mut robot = Self::find_robot(&map);
        map[robot.y][robot.x] = '.';

        for m in moves.chars() {
            let next = robot.next(m);

            let e = map[next.y][next.x];
            if e == '.' {
                robot = next;
            } else if e == '[' || e == ']' {
                let mut visited: Vec<Point<usize>> = Vec::with_capacity(16);
                let mut can_move = true;

                if next.y == robot.y {
                    // moving horizontally
                    visited.push(next.clone());
                    let mut next_box = next.next(m);
                    while map[next_box.y][next_box.x] == '[' || map[next_box.y][next_box.x] == ']' {
                        visited.push(next_box.clone());
                        next_box = next_box.next(m);
                    }

                    can_move = map[next_box.y][next_box.x] == '.';
                } else {
                    // Moving vertically
                    let mut togo: Vec<Point<usize>> = Vec::with_capacity(16);
                    togo.push(next.clone());

                    while let Some(p) = togo.pop() {
                        if visited.contains(&p) {
                            continue;
                        }

                        visited.push(p.clone());
                        if map[p.y][p.x] == '[' {
                            togo.push(p.right());
                        } else {
                            togo.push(p.left());
                        }

                        let n = p.next(m);
                        match map[n.y][n.x] {
                            '[' => togo.push(n),
                            ']' => togo.push(n),
                            '#' => {
                                can_move = false;
                                break;
                            }
                            _ => {}
                        }
                    }
                }

                if can_move {
                    if m == '^' {
                        visited.sort_by_key(|e| 0xFFFF - e.y);
                    } else if m == 'v' {
                        visited.sort_by_key(|e| e.y);
                    }

                    for p in visited.iter().rev() {
                        let n = p.next(m);
                        map[n.y][n.x] = map[p.y][p.x];
                        map[p.y][p.x] = '.';
                    }

                    robot = next;
                }
            }
        }

        Self::get_gps_positions(&map, '[')
    }

    fn read_input(&self, filename: &str) -> (Vec<Vec<char>>, String) {
        let data = self.read_input_file(filename);
        let (map, moves) = data.split_once("\n\n").unwrap();
        (
            map.split('\n').map(|line| line.chars().collect()).collect(),
            String::from(moves).replace("\n", ""),
        )
    }

    fn find_robot(map: &Vec<Vec<char>>) -> Point<usize> {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '@' {
                    return Point { x, y };
                }
            }
        }

        panic!("No robot found");
    }

    fn get_gps_positions(map: &Vec<Vec<char>>, b: char) -> usize {
        let mut result = 0usize;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == b {
                    result += y * 100 + x;
                }
            }
        }

        result
    }

    fn expand_map(&self, map: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
        map.iter()
            .map(|row| {
                let mut new_row: Vec<char> = Vec::with_capacity(row.len() * 2);
                for e in row {
                    match e {
                        '#' => new_row.append(&mut vec!['#', '#']),
                        '.' => new_row.append(&mut vec!['.', '.']),
                        '@' => new_row.append(&mut vec!['@', '.']),
                        'O' => new_row.append(&mut vec!['[', ']']),
                        _ => panic!("Invalid field: {}", e),
                    }
                }

                new_row
            })
            .collect()
    }
}
