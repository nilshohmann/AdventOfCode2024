use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub trait Parsing {
    fn to<F: FromStr + Debug>(self) -> F
    where
        F::Err: Debug;
}

impl Parsing for &str {
    fn to<F: FromStr + Debug>(self) -> F
    where
        F::Err: Debug,
    {
        FromStr::from_str(self).unwrap()
    }
}

pub trait ListParsing {
    fn parse_as<F: FromStr + Debug>(self) -> Vec<F>
    where
        F::Err: Debug;
}

impl<'a, T: Iterator<Item=&'a str>> ListParsing for T {
    fn parse_as<F: FromStr + Debug>(self) -> Vec<F>
    where
        F::Err: Debug,
    {
        self.map(|e| e.to()).collect()
    }
}
