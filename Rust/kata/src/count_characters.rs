use std::collections::HashMap;
use std::str::FromStr;

// https://www.codewars.com/kata/52efefcbcdf57161d4000091/train/rust

struct Counter<T> {
    map: HashMap<T, i32>,
}

impl<T> Counter<T>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, item: T) {
        let count = self.map.entry(item).or_insert(0);
        *count += 1;
    }
}

impl FromStr for Counter<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counter = Self::new();
        for c in s.chars() {
            counter.add(c);
        }
        Ok(counter)
    }
}

impl<T> From<Counter<T>> for HashMap<T, i32> {
    fn from(counter: Counter<T>) -> Self {
        counter.map
    }
}

fn count(input: &str) -> HashMap<char, i32> {
    input
        .parse::<Counter<char>>()
        .expect("Failed to parse input")
        .into()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn test_empty_string() {
        let test_input = "";
        let expected: HashMap<char, i32> = HashMap::new();

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_two_equal_letters() {
        let test_input = "aa";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_different_letters() {
        let test_input = "aabb";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }
}
