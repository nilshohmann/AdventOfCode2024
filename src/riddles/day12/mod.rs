use std::collections::HashSet;
use crate::riddles::{Riddle, Utils};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn top(&self) -> Point { Point { x: self.x, y: self.y - 1 } }
    fn right(&self) -> Point { Point { x: self.x + 1, y: self.y } }
    fn bottom(&self) -> Point { Point { x: self.x, y: self.y + 1 } }
    fn left(&self) -> Point { Point { x: self.x - 1, y: self.y } }

    fn around(&self) -> [Point; 4] {
        [
            self.top(),
            self.right(),
            self.bottom(),
            self.left(),
        ]
    }
}

pub struct Day12();

impl Riddle for Day12 {
    fn day(&self) -> u8 { 12 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 1930)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 1206)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day12 {
    fn _solve_first(&self, filename: &str) -> usize {
        let map = self.read_map(filename);

        let mut visited: HashSet<Point> = HashSet::with_capacity(map.len() * map[0].len());

        let mut result = 0usize;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let start = Point { x: x as i32, y: y as i32 };
                if visited.contains(&start) {
                    continue;
                }

                let id = Self::field_at(&start, &map);
                let mut field: Vec<Point> = vec![start];

                let mut area = 0usize;
                let mut perimeter = 0usize;

                while let Some(p) = field.pop() {
                    if visited.contains(&p) {
                        continue;
                    }

                    visited.insert(p.clone());
                    area += 1;

                    for p in p.around() {
                        if Self::field_at(&p, &map) != id {
                            perimeter += 1;
                        } else if !visited.contains(&p) {
                            field.push(p);
                        }
                    }
                }

                result += area * perimeter;
            }
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> usize {
        let map = self.read_map(filename);

        let mut visited: HashSet<Point> = HashSet::with_capacity(map.len() * map[0].len());

        let mut result = 0usize;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let start = Point { x: x as i32, y: y as i32 };
                if visited.contains(&start) {
                    continue;
                }

                let id = Self::field_at(&start, &map);
                let mut field: Vec<Point> = vec![start];

                let mut area = 0usize;
                let mut fences: [Vec<Point>; 4] = [const { Vec::new() }; 4];

                while let Some(p) = field.pop() {
                    if visited.contains(&p) {
                        continue;
                    }

                    visited.insert(p.clone());
                    area += 1;

                    for (i, p) in p.around().iter().enumerate() {
                        if Self::field_at(p, &map) != id {
                            fences[i].push(p.clone());
                        } else if !visited.contains(&p) {
                            field.push(p.clone());
                        }
                    }
                }

                let sides = Self::find_sides(&fences);

                result += sides * area;
            }
        }

        result
    }

    fn find_sides(fences: &[Vec<Point>; 4]) -> usize {
        let mut result = 0usize;

        for i in 0..fences.len() {
            let mut visited: HashSet<Point> = HashSet::with_capacity(fences[i].len());

            for p in fences[i].iter() {
                if visited.contains(p) {
                    continue;
                }

                visited.insert(p.clone());
                result += 1;

                if i & 1 == 1 { // fences left and right -> look vertical
                    let mut togo = vec![p.top(), p.bottom()];

                    while let Some(p) = togo.pop() {
                        if !visited.contains(&p) && fences[i].contains(&p) {
                            visited.insert(p.clone());
                            togo.push(p.top());
                            togo.push(p.bottom());
                        }
                    }
                } else { // fences top and bottom -> look horizontal
                    let mut togo = vec![p.left(), p.right()];

                    while let Some(p) = togo.pop() {
                        if !visited.contains(&p) && fences[i].contains(&p) {
                            visited.insert(p.clone());
                            togo.push(p.left());
                            togo.push(p.right());
                        }
                    }
                }
            }
        }

        result
    }

    fn field_at(p: &Point, map: &Vec<Vec<char>>) -> char {
        if p.y < 0 || p.y >= map.len() as i32 || p.x < 0 || p.x >= map[0].len() as i32 {
            return '-';
        }

        map[p.y as usize][p.x as usize]
    }
}
