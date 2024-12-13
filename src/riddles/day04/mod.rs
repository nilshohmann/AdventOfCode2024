use crate::riddles::Riddle;
use crate::riddles::utils::Utils;

pub struct Day04();

impl Riddle for Day04 {
    fn day(&self) -> u8 { 4 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 18)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 9)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day04 {
    fn _solve_first(&self, filename: &str) -> i32 {
        let board = self.read_map(filename);

        let mut result: i32 = 0;

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                result += self.find_xmas(&board, y, x);
            }
        }

        result
    }

    fn _solve_second(&self, filename: &str) -> i32 {
        let board = self.read_map(filename);

        let mut result: i32 = 0;

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                result += self.find_x_mas(&board, y, x);
            }
        }

        result
    }

    fn find_xmas(&self, board: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
        let c = board[y][x];
        if c != 'X' {
            return 0;
        }

        let mut result = 0;

        if x >= 3 {
            if self.is_mas(board[y][x - 1], board[y][x - 2], board[y][x - 3]) {
                result += 1;
            }

            if y >= 3 && self.is_mas(board[y - 1][x - 1], board[y - 2][x - 2], board[y - 3][x - 3]) {
                result += 1;
            }

            if y < board.len() - 3 && self.is_mas(board[y + 1][x - 1], board[y + 2][x - 2], board[y + 3][x - 3]) {
                result += 1;
            }
        }

        if x < board[y].len() - 3 {
            if self.is_mas(board[y][x + 1], board[y][x + 2], board[y][x + 3]) {
                result += 1;
            }

            if y >= 3 && self.is_mas(board[y - 1][x + 1], board[y - 2][x + 2], board[y - 3][x + 3]) {
                result += 1;
            }

            if y < board.len() - 3 && self.is_mas(board[y + 1][x + 1], board[y + 2][x + 2], board[y + 3][x + 3]) {
                result += 1;
            }
        }

        if y >= 3 && self.is_mas(board[y - 1][x], board[y - 2][x], board[y - 3][x]) {
            result += 1;
        }

        if y < board.len() - 3 && self.is_mas(board[y + 1][x], board[y + 2][x], board[y + 3][x]) {
            result += 1;
        }

        result
    }

    fn is_mas(&self, char1: char, char2: char, char3: char) -> bool {
        char1 == 'M' && char2 == 'A' && char3 == 'S'
    }

    fn find_x_mas(&self, board: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
        let c = board[y][x];
        if c != 'A' || y == 0 || y == board.len() - 1 || x == 0 || x == board[y].len() - 1 {
            return 0;
        }

        if self.is_ms(board[y - 1][x - 1], board[y + 1][x + 1]) &&
            self.is_ms(board[y - 1][x + 1], board[y + 1][x - 1]) {
            return 1;
        }
        0
    }

    fn is_ms(&self, char1: char, char2: char) -> bool {
        (char1 == 'M' && char2 == 'S') ||
            (char1 == 'S' && char2 == 'M')
    }
}
