use crate::riddles::utils::{Parsing, Point, Utils};
use crate::riddles::Riddle;

#[derive(Debug)]
struct Robot {
    position: Point<i32>,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn forward(&mut self, times: i32, width: usize, height: usize) {
        let (height, width) = (height as i32, width as i32);

        self.position.x = (self.position.x + times * self.vx) % width;
        if self.position.x < 0 {
            self.position.x += width;
        }

        self.position.y = (self.position.y + times * self.vy) % height;
        if self.position.y < 0 {
            self.position.y += height;
        }
    }
}

pub struct Day14();

impl Riddle for Day14 {
    fn day(&self) -> u8 {
        14
    }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt", 11, 7), 12)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt", 101, 103).to_string()
    }

    fn validate_second(&self) -> bool {
        true // Not happening for test input
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt", 101, 103).to_string()
    }
}

impl Day14 {
    fn _solve_first(&self, filename: &str, width: usize, height: usize) -> i32 {
        let mut robots = self.read_robots(filename);

        let mid_x = (width >> 1) as i32;
        let mid_y = (height >> 1) as i32;

        let mut counts: [[i32; 2]; 2] = [[0, 0], [0, 0]];
        for robot in robots.iter_mut() {
            robot.forward(100, width, height);

            if robot.position.x != mid_x && robot.position.y != mid_y {
                counts[(robot.position.y / (mid_y + 1)) as usize]
                    [(robot.position.x / (mid_x + 1)) as usize] += 1;
            }
        }

        counts[0][0] * counts[0][1] * counts[1][0] * counts[1][1]
    }

    fn _solve_second(&self, filename: &str, width: usize, height: usize) -> usize {
        let mut robots = self.read_robots(filename);
        let mut i = 0usize;

        loop {
            i += 1;

            for robot in robots.iter_mut() {
                robot.forward(1, width, height);
            }

            if Self::shows_christmas_tree(&robots, width, height) {
                return i;
            }
        }
    }

    fn read_robots(&self, filename: &str) -> Vec<Robot> {
        self.read_input_file(filename)
            .split('\n')
            .map(|line| {
                let (p, v) = line.split_once(" ").unwrap();
                let (px, py) = p.split_once("=").unwrap().1.split_once(",").unwrap();
                let (vx, vy) = v.split_once("=").unwrap().1.split_once(",").unwrap();
                Robot {
                    position: Point {
                        x: px.to::<i32>(),
                        y: py.to::<i32>(),
                    },
                    vx: vx.to(),
                    vy: vy.to(),
                }
            })
            .collect()
    }

    fn shows_christmas_tree(robots: &Vec<Robot>, width: usize, height: usize) -> bool {
        // Look for the tip of the Christmas tree (reversed here)
        let target = [
            ".#########.",
            "..#######..",
            "...#####...",
            "....###....",
            ".....#.....",
            "...........",
        ];

        let lines = Self::print_robots(&robots, width, height);
        for (row, line) in lines.iter().enumerate().skip(target.len() - 1) {
            if let Some(index) = line.find(target[0]) {
                let mut found = true;

                for i in 1..target.len() {
                    if !lines[row - i][index..].starts_with(target[i]) {
                        found = false;
                        break;
                    }
                }

                if found {
                    return true;
                }
            }
        }

        false
    }

    fn print_robots(robots: &Vec<Robot>, width: usize, height: usize) -> Vec<String> {
        let mut field: Vec<Vec<char>> = vec![vec!['.'; width]; height];

        for robot in robots {
            field[robot.position.y as usize][robot.position.x as usize] = '#';
        }

        field
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect()
    }
}
