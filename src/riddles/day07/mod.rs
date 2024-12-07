use crate::riddles::{expect, Riddle};

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

const ADD: u8 = 0;
const MULTIPLY: u8 = 1;
const JOIN: u8 = 2;

pub struct Day07();

impl Riddle for Day07 {
    fn day(&self) -> u8 { 7 }

    fn validate_first(&self) -> bool {
        expect(self._solve_first("input_test.txt"), 3749)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        expect(self._solve_second("input_test.txt"), 11387)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day07 {
    fn _solve_first(&self, filename: &str) -> u64 {
        let equations = self.read_equations(filename);

        let mut result = 0;

        for equation in equations.iter() {
            if self.can_be_solved(equation, 2) {
                result += equation.result;
            }
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> u64 {
        let equations = self.read_equations(filename);

        let mut result = 0;

        for equation in equations.iter() {
            if self.can_be_solved(equation, 3) {
                result += equation.result;
            }
        }

        result
    }

    fn read_equations(&self, filename: &str) -> Vec<Equation> {
        self.read_input_file(filename).split("\n")
            .map(|s| {
                let (left, right) = s.split_once(": ").unwrap();
                Equation {
                    result: left.parse::<u64>().unwrap(),
                    values: right.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect(),
                }
            }).collect::<Vec<Equation>>()
    }

    fn can_be_solved(&self, equation: &Equation, base: u8) -> bool {
        let mut operations: Vec<u8> = vec![0; equation.values.len() - 1];
        if self.can_be_solved_with(equation, &operations) {
            return true;
        }

        let count = (base as u64).pow(equation.values.len() as u32 - 1);
        for _ in 0..count {
            for i in 0..(equation.values.len() - 1) {
                operations[i] = (operations[i] + 1) % base;
                if operations[i] != 0 {
                    break;
                }
            }

            if self.can_be_solved_with(equation, &operations) {
                return true;
            }
        }

        false
    }

    fn can_be_solved_with(&self, equation: &Equation, operations: &Vec<u8>) -> bool {
        let mut result = equation.values[0];

        for (i, value) in equation.values[1..].iter().enumerate() {
            match operations[i] {
                ADD => result += value,
                MULTIPLY => result *= value,
                JOIN => result = (result.to_string() + value.to_string().as_str()).parse::<u64>().unwrap(),
                _ => panic!("Unexpected operation: {}", operations[i]),
            }

            if result > equation.result {
                return false;
            }
        }

        result == equation.result
    }
}