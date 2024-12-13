use std::fmt::Display;

mod extensions;
mod point;

pub use extensions::*;
pub use point::*;

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
