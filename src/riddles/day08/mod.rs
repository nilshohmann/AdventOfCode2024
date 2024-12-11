use std::collections::{HashMap, HashSet};
use crate::riddles::{Riddle, Utils};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Day08();

impl Riddle for Day08 {
    fn day(&self) -> u8 { 8 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 14)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 34)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day08 {
    fn _solve_first(&self, filename: &str) -> usize {
        let map = self.read_map(filename);
        let all_antennas = self.find_antennas(&map);

        self.count_antinodes(&map, &all_antennas, false)
    }

    fn _solve_second(&self, filename: &str) -> usize {
        let map = self.read_map(filename);
        let all_antennas = self.find_antennas(&map);

        self.count_antinodes(&map, &all_antennas, true)
    }

    fn find_antennas(&self, map: &Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != '.' {
                    let point = Point { x: x as i32, y: y as i32 };

                    if antennas.contains_key(&map[y][x]) {
                        antennas.get_mut(&map[y][x]).unwrap().push(point);
                    } else {
                        antennas.insert(map[y][x], vec![point]);
                    }
                }
            }
        }

        antennas
    }

    fn count_antinodes(&self,
                       map: &Vec<Vec<char>>,
                       all_antennas: &HashMap<char, Vec<Point>>,
                       resonates: bool) -> usize {
        let mut antinodes: HashSet<Point> = HashSet::new();
        let mut add_antinode = |p: Point| -> bool {
            if p.x >= 0 && p.y >= 0 && p.x < map.len() as i32 && p.y < map[0].len() as i32 {
                antinodes.insert(p);
                return true;
            }
            false
        };

        for (_, antennas) in all_antennas.iter() {
            for i in 0..(antennas.len() - 1) {
                for j in (i + 1)..antennas.len() {
                    let dx = antennas[i].x - antennas[j].x;
                    let dy = antennas[i].y - antennas[j].y;

                    if resonates {
                        for m in 0..0xFF {
                            if !add_antinode(Point {
                                x: antennas[i].x + m * dx,
                                y: antennas[i].y + m * dy,
                            }) {
                                break;
                            }
                        }
                        for m in 0..0xFF {
                            if !add_antinode(Point {
                                x: antennas[j].x - m * dx,
                                y: antennas[j].y - m * dy,
                            }) {
                                break;
                            }
                        }
                    } else {
                        add_antinode(Point { x: antennas[i].x + dx, y: antennas[i].y + dy });
                        add_antinode(Point { x: antennas[j].x - dx, y: antennas[j].y - dy });
                    }
                }
            }
        }

        antinodes.len()
    }
}
