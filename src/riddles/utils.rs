use std::fmt::Display;

pub struct Utils {}

impl Utils {
    pub fn verify<T: Eq + Display>(result: T, expected: T) -> bool {
        if expected != result {
            println!("Expected {}, got {}", expected, result);
            return false;
        }

        true
    }
}
