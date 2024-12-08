pub mod day01;
pub use day01::Day01;
pub mod day02;
pub use day02::Day02;
pub mod day03;
pub use day03::Day03;
pub mod day04;
pub use day04::Day04;
pub mod day05;
pub use day05::Day05;
pub mod day06;
pub use day06::Day06;
pub mod day07;
pub use day07::Day07;
pub mod day08;
pub use day08::Day08;
pub mod day09;
pub use day09::Day09;
pub mod day10;
pub use day10::Day10;
pub mod day11;
pub use day11::Day11;
pub mod day12;
pub use day12::Day12;
pub mod day13;
pub use day13::Day13;
pub mod day14;
pub use day14::Day14;
pub mod day15;
pub use day15::Day15;
pub mod day16;
pub use day16::Day16;
pub mod day17;
pub use day17::Day17;
pub mod day18;
pub use day18::Day18;
pub mod day19;
pub use day19::Day19;
pub mod day20;
pub use day20::Day20;
pub mod day21;
pub use day21::Day21;
pub mod day22;
pub use day22::Day22;
pub mod day23;
pub use day23::Day23;
pub mod day24;
pub use day24::Day24;
pub mod day25;
pub use day25::Day25;

use std::env;
use std::fmt::Display;
use std::fs;
use std::time::{Duration, Instant};

pub trait Riddle {
    fn day(&self) -> u8;

    fn validate_first(&self) -> bool { false }
    fn solve_first(&self) -> String { String::new() }

    fn validate_second(&self) -> bool { false }
    fn solve_second(&self) -> String { String::new() }

    fn execute(&self) -> String {
        let (r1, d1) = if self.validate_first() {
            measure_duration(|| self.solve_first())
        } else {
            ("Validation failed".to_string(), Duration::from_secs(0))
        };

        let (r2, d2) = if self.validate_second() {
            measure_duration(|| self.solve_second())
        } else {
            ("Validation failed".to_string(), Duration::from_secs(0))
        };

        let content = [
            "# Results",
            "| Part | Result | Time |",
            "| --- | --- | --- |",
            &format!("| 1 | {} | {} |", r1, format_duration(d1)),
            &format!("| 2 | {} | {} |", r2, format_duration(d2)),
        ];

        content.join("\n")
    }

    fn read_input_file(&self, name: &str) -> String {
        let mut path = env::current_dir()
            .expect("Could not get current directory");
        path.push("src");
        path.push("riddles");
        path.push(format!("day{:0>2}", self.day()));
        path.push(name);

        let content = fs::read_to_string(&path);

        match content {
            Ok(content) => content,
            Err(err) => panic!("Could not read file {}: {}", path.to_str().unwrap(), err),
        }
    }

    fn read_map(&self, filename: &str) -> Vec<Vec<char>> {
        self.read_input_file(filename)
            .split("\n")
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }
}

pub fn expect<T: Eq + Display>(result: T, expected: T) -> bool {
    if expected != result {
        println!("Expected {}, got {}", expected, result);
        return false;
    }

    true
}

fn measure_duration(f: impl Fn() -> String) -> (String, Duration) {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    (result, duration)
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();

    let value: u64;
    let unit: &str;

    if seconds == 0 {
        value = duration.subsec_micros() as u64 / 10;
        unit = "ms";
    } else if seconds < 60 {
        value = seconds * 100 + duration.subsec_millis() as u64 / 10;
        unit = "s";
    } else {
        value = (seconds * 10 + duration.subsec_millis() as u64 / 100) / 6;
        unit = "min";
    }

    format!("{}{}", (value as f64) / 100.0, unit)
}