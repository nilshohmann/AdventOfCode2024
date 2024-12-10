use std::collections::HashSet;
use crate::riddles::{expect, Riddle};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

pub struct Day10();

impl Riddle for Day10 {
    fn day(&self) -> u8 { 10 }

    fn validate_first(&self) -> bool {
        expect(self._solve_first("input_test.txt"), 36)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        expect(self._solve_second("input_test.txt"), 81)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day10 {
    fn _solve_first(&self, filename: &str) -> usize {
        let map = self.read_map(filename);

        let mut result = 0;

        for start in Self::find_starting_points(&map, '0').iter() {
            let score = Self::find_trail_score(&map, start);
            result += score;
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> usize {
        let map = self.read_map(filename);

        let mut result = 0;

        for start in Self::find_starting_points(&map, '9').iter() {
            let score = Self::find_trail_rating(&map, start);
            result += score;
        }

        result
    }

    fn find_starting_points(map: &Vec<Vec<char>>, value: char) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == value {
                    points.push(Point { x, y });
                }
            }
        }

        points
    }

    fn find_trail_score(map: &Vec<Vec<char>>, start: &Point) -> usize {
        let mut next_points: Vec<Point> = vec![start.clone()];
        let mut visited: HashSet<Point> = HashSet::new();

        let mut result = 0;
        while let Some(p) = next_points.pop() {
            if visited.contains(&p) {
                continue;
            }

            visited.insert(p.clone());

            let value = map[p.y][p.x];
            if value == '9' {
                result += 1;
            }

            let next_value = (value as u8 + 1) as char;
            if p.x > 0 && map[p.y][p.x - 1] == next_value {
                next_points.push(Point { x: p.x - 1, y: p.y });
            }
            if p.y > 0 && map[p.y - 1][p.x] == next_value {
                next_points.push(Point { x: p.x, y: p.y - 1 });
            }
            if p.x < map[p.y].len() - 1 && map[p.y][p.x + 1] == next_value {
                next_points.push(Point { x: p.x + 1, y: p.y });
            }
            if p.y < map.len() - 1 && map[p.y + 1][p.x] == next_value {
                next_points.push(Point { x: p.x, y: p.y + 1 });
            }
        }

        result
    }

    fn find_trail_rating(map: &Vec<Vec<char>>, p: &Point) -> usize {
        let value = map[p.y][p.x];
        if value == '0' {
            return 1;
        }

        let mut result = 0;
        let next_value = (value as u8 - 1) as char;

        if p.x > 0 && map[p.y][p.x - 1] == next_value {
            result += Self::find_trail_rating(map, &Point { x: p.x - 1, y: p.y });
        }
        if p.y > 0 && map[p.y - 1][p.x] == next_value {
            result += Self::find_trail_rating(map, &Point { x: p.x, y: p.y - 1 });
        }
        if p.x < map[p.y].len() - 1 && map[p.y][p.x + 1] == next_value {
            result += Self::find_trail_rating(map, &Point { x: p.x + 1, y: p.y });
        }
        if p.y < map.len() - 1 && map[p.y + 1][p.x] == next_value {
            result += Self::find_trail_rating(map, &Point { x: p.x, y: p.y + 1 });
        }

        result
    }
}
