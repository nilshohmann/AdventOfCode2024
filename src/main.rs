use std::env;
use std::io::{stdin, stdout, Write};

mod riddles;
use crate::riddles::Riddle;

fn main() {
    let days: Vec<&dyn Riddle> = vec![
        &riddles::Day01(),
        &riddles::Day02(),
        &riddles::Day03(),
        &riddles::Day04(),
        &riddles::Day05(),
        &riddles::Day06(),
        &riddles::Day07(),
        &riddles::Day08(),
        &riddles::Day09(),
        &riddles::Day10(),
        &riddles::Day11(),
        &riddles::Day12(),
        &riddles::Day13(),
        &riddles::Day14(),
        &riddles::Day15(),
        &riddles::Day16(),
        &riddles::Day17(),
        &riddles::Day18(),
        &riddles::Day19(),
        &riddles::Day20(),
        &riddles::Day21(),
        &riddles::Day22(),
        &riddles::Day23(),
        &riddles::Day24(),
        &riddles::Day25(),
    ];

    let day = get_day();

    println!("Solving day {}", day);
    let result = days[day - 1].execute();
    println!("{}", result);
}

fn get_day() -> usize {
    if let Some(day) = env::var("DAY").ok().and_then(|v| v.parse::<usize>().ok()) {
        return day
    }

    loop {
        if let Some(day) = read_day() {
            return day;
        } else {
            println!("Please enter a valid day!");
        }
    }
}

fn read_day() -> Option<usize> {
    let mut s = String::new();

    print!("Which day do you want to solve [1-25]? ");
    let _ = stdout().flush();

    let input = match stdin().read_line(&mut s) {
        Ok(_) => {
            s.pop();
            s.parse::<usize>().ok()
        },
        _ => None,
    };

    match input {
        Some(day) => if day >= 1 && day <= 25 { Some(day) } else { None },
        _ => None,
    }
}