use crate::riddles::utils::{ListParsing, Parsing, Utils};
use crate::riddles::Riddle;

pub struct Day17();

impl Riddle for Day17 {
    fn day(&self) -> u8 {
        17
    }

    fn validate_first(&self) -> bool {
        Utils::verify(
            self._solve_first("input_test_1.txt"),
            String::from("4,2,5,6,7,7,7,7,3,1,0"),
        )
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test_2.txt"), 117440)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day17 {
    fn _solve_first(&self, filename: &str) -> String {
        let (registers, instructions) = self.read_data(filename);

        ProgramSolver::new(registers, &instructions)
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn _solve_second(&self, filename: &str) -> u64 {
        let (_, instructions) = self.read_data(filename);

        let target = instructions.iter().rev().collect::<Vec<&u8>>();

        let mut results: Vec<u64> = Vec::new();
        let mut states: Vec<(u8, u64)> = vec![(0, 0)];

        while let Some((i, a)) = states.pop() {
            for o in 0..8 {
                let a = (a << 3) | o;
                let r = ProgramSolver::new([a, 0, 0], &instructions).collect::<Vec<u8>>();

                if r.iter().rev().zip(&target).all(|(a, b)| a == *b) {
                    if r.len() == target.len() {
                        results.push(a);
                    } else if i < target.len() as u8 {
                        states.push((i + 1, a))
                    }
                }
            }
        }

        *results.iter().min().unwrap()
    }

    fn read_data(&self, filename: &str) -> ([u64; 3], Vec<u8>) {
        let data = self.read_input_file(filename);
        let (left, right) = data.split_once("\n\n").unwrap();

        let mut registers = [0; 3];
        left.split('\n')
            .enumerate()
            .for_each(|(i, e)| registers[i] = e.split_once(": ").unwrap().1.to::<u64>());

        let instructions = right
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .parse_as::<u8>();

        (registers, instructions)
    }
}

struct ProgramSolver<'a> {
    pub registers: [u64; 3],
    pub instructions: &'a Vec<u8>,
    pointer: usize,
}

impl<'a> ProgramSolver<'a> {
    pub fn new(registers: [u64; 3], instructions: &'a Vec<u8>) -> Self {
        ProgramSolver {
            registers,
            instructions,
            pointer: 0,
        }
    }

    fn current_operand(&self) -> u8 {
        self.instructions[self.pointer + 1]
    }

    fn current_combo_operand(&self) -> u64 {
        let op = self.instructions[self.pointer + 1] as u64;
        match op {
            7 => panic!("Invalid combo operand at {}", self.pointer + 1),
            6 => self.registers[2],
            5 => self.registers[1],
            4 => self.registers[0],
            _ => op,
        }
    }
}

impl<'a> Iterator for ProgramSolver<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pointer < self.instructions.len() {
            match self.instructions[self.pointer] {
                0 /* ADV */ => self.registers[0] = &self.registers[0] >> self.current_combo_operand(),
                1 /* BXL */ => self.registers[1] = &self.registers[1] ^ (self.current_operand() as u64),
                2 /* BST */ => self.registers[1] = self.current_combo_operand() & 0x7,
                3 /* JNZ */ => {
                    if self.registers[0] != 0 {
                        self.pointer = self.current_operand() as usize;
                        continue;
                    }
                }
                4 /* BXC */ => self.registers[1] = self.registers[1] ^ self.registers[2],
                5 /* OUT */ => {
                    let output = (self.current_combo_operand() & 0x7) as u8;
                    self.pointer += 2;
                    return Some(output);
                }
                6 /* BDV */ => self.registers[1] = self.registers[0] >> self.current_combo_operand(),
                7 /* CDV */ => self.registers[2] = self.registers[0] >> self.current_combo_operand(),
                _ => panic!("Invalid instruction: {}", self.instructions[self.pointer]),
            }

            self.pointer += 2;
        }

        None
    }
}
