/*
https://www.codewars.com/kata/59ca8e8e1a68b7de740001f4

DESCRIPTION:
Given two arrays of strings, return the number of times each string of the second array appears in the first array.

Example
array1 = ['abc', 'abc', 'xyz', 'cde', 'uvw']
array2 = ['abc', 'cde', 'uap']
How many times do the elements in array2 appear in array1?

'abc' appears twice in the first array (2)
'cde' appears only once (1)
'uap' does not appear in the first array (0)
Therefore, solve(array1, array2) = [2, 1, 0]

Good luck!

If you like this Kata, please try:
*/

use std::collections::HashMap;
struct Counter<'a>(HashMap<&'a str, usize>);

impl<'a> Counter<'a> {
    fn new() -> Self {
        Counter(HashMap::new())
    }

    fn create(val: &'a [String]) -> Self {
        let mut map = Self::new();
        val.iter().for_each(|v| map.add(v));
        map
    }

    fn add(&mut self, key: &'a str) {
        *self.0.entry(key).or_insert(0) += 1;
    }

    fn get(&self, key: &'a str) -> usize {
        *self.0.get(key).unwrap_or(&0)
    }
}

fn match_counts(a1: &[String], a2: &[String]) -> Vec<usize> {
    let counter = Counter::create(a1);
    a2.iter().map(|v| counter.get(v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            match_counts(
                &[
                    "abc".to_string(),
                    "abc".to_string(),
                    "xyz".to_string(),
                    "abcd".to_string(),
                    "cde".to_string()
                ],
                &["abc".to_string(), "cde".to_string(), "uap".to_string()]
            ),
            vec![2, 1, 0]
        );
        assert_eq!(
            match_counts(
                &[
                    "abc".to_string(),
                    "xyz".to_string(),
                    "abc".to_string(),
                    "xyz".to_string(),
                    "cde".to_string()
                ],
                &["abc".to_string(), "cde".to_string(), "xyz".to_string()]
            ),
            vec![2, 1, 2]
        );
        assert_eq!(
            match_counts(
                &[
                    "quick".to_string(),
                    "brown".to_string(),
                    "fox".to_string(),
                    "is".to_string(),
                    "quick".to_string()
                ],
                &["quick".to_string(), "abc".to_string(), "fox".to_string()]
            ),
            vec![2, 0, 1]
        );
    }
}
