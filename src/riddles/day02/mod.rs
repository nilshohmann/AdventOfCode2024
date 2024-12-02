use crate::riddles::Riddle;

pub struct Day02();

impl Riddle for Day02 {
    fn day(&self) -> u8 { 2 }

    fn solve_first(&self) -> String {
        let reports = self.read_reports();
        let result = reports.iter().filter(|e| self.is_safe(e)).count();
        result.to_string()
    }

    fn solve_second(&self) -> String {
        let reports = self.read_reports();
        let result = reports.iter().filter(|e| self.is_safe_with_tolerance(&e)).count();
        result.to_string()
    }
}

impl Day02 {
    fn read_reports(&self) -> Vec<Vec<i32>> {
        self.read_input_file("input.txt")
            .split("\n")
            .map(|line| line.split_whitespace().map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .collect()
    }

    fn is_safe(&self, report: &Vec<i32>) -> bool {
        let mut safe_count = 0;

        let mut p = &report[0];
        for c in report.iter().skip(1) {
            if c > p && (c - p) <= 3 && safe_count >= 0 {
                safe_count = 1;
            } else if c < p && (p - c) <= 3 && safe_count <= 0 {
                safe_count = -1;
            } else {
                return false;
            }

            p = c;
        }

        true
    }

    fn is_safe_with_tolerance(&self, report: &Vec<i32>) -> bool {
        if self.is_safe(report) {
            return true
        }

        for n in 0..report.len() {
            let adjusted_report = report.into_iter()
                .enumerate()
                .filter_map(|(i, e)| if i != n { Some(e) } else { None })
                .map(|e| *e)
                .collect::<Vec<i32>>();

            if self.is_safe(&adjusted_report) {
                return true
            }
        }

        false
    }
}
