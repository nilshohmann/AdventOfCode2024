use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;
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

pub trait HashMapExt<K, V> {
    fn add(&mut self, value: V, key: K);
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V> where V: Clone + Add<Output=V>, K: Hash + Eq + Clone {
    fn add(&mut self, value: V, key: K) {
        if self.contains_key(&key) {
            let value = self[&key].clone() + value;
            self.insert(key, value);
        } else {
            self.insert(key, value);
        }
    }
}
