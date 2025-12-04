use itertools::Itertools;
use std::str::FromStr;
use std::fmt::Write;

struct ProductIdRange {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Error {
    NotValidRange,
}
impl FromStr for ProductIdRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();
        Ok(ProductIdRange { start, end })
    }
}

impl ProductIdRange {
    fn get_invalid_product_ids(&self) -> Vec<usize> {
        (self.start..=self.end)
            .filter(|&i| ProductId(&i.to_string()).simple_pattern_inside())
            .collect()
    }
    fn get_invalid_product_ids_2(&self) -> Vec<usize> {
        let mut buf = String::with_capacity(20);
        (self.start..=self.end)
            .filter(|&i| {
                buf.clear();
                let _ = write!(buf, "{}", i);
                ProductId(&buf).repeating_pattern_inside()
            })
            .collect()
    }
}

struct ProductIdRanges(Vec<ProductIdRange>);

impl FromStr for ProductIdRanges {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|r| r.parse::<ProductIdRange>())
            .collect::<Result<_, _>>()
            .map(ProductIdRanges)
    }
}

struct ProductId<'a>(&'a str);

impl<'a> ProductId<'a> {
    fn simple_pattern_inside(&self) -> bool {
        if self.0.len() % 2 == 1 {
            return false;
        }

        let pivot = self.0.len() / 2;
        let left = &self.0[..pivot];
        let right = &self.0[pivot..];

        left == right
    }

    fn repeating_pattern_inside(&self) -> bool {
        let n = self.0.len();
        for i in 1..=n / 2 {
            // i is a divisor of n
            if n % i == 0 && self.0[i..] == self.0[..n - i] {
                return true;
            }
        }
        false
    }
}

fn part1(input: &str) -> usize {
    let ranges = ProductIdRanges::from_str(input).unwrap();
    ranges
        .0
        .iter()
        .map(|r| r.get_invalid_product_ids())
        .flatten()
        .sum()
}

fn part2(input: &str) -> usize {
    let ranges = ProductIdRanges::from_str(input).unwrap();
    ranges
        .0
        .iter()
        .map(|r| r.get_invalid_product_ids_2())
        .flatten()
        .sum()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day2.txt");
    format!("Day 2\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 1227775554);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 4174379265);
    }
}
