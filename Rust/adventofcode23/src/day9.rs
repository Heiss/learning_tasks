use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Value(isize);

impl Deref for Value {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct ValueHistory {
    values: Vec<Value>,
    differences: Vec<Vec<Value>>,
}

impl FromStr for ValueHistory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(|v| v.parse().map(Value).unwrap())
            .collect::<Vec<Value>>();

        Ok(Self {
            values,
            differences: Vec::new(),
        })
    }
}

impl ValueHistory {
    fn generate_diff(&mut self) {
        let mut line_differences = Vec::new();
        line_differences.push(self.values.clone());

        let mut current = self.values.clone();
        while !current.iter().all(|v| v.0 == 0) {
            current = current.windows(2).map(|v| Value(v[1].0 - v[0].0)).collect();
            line_differences.push(current.clone());
        }

        self.differences = line_differences;
    }
}

struct Sensor {
    histories: Vec<ValueHistory>,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let histories = s
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<ValueHistory>>();

        Ok(Self { histories })
    }
}

impl Sensor {
    fn get_next_value_sum(&mut self) -> isize {
        self.histories.iter_mut().for_each(|h| h.generate_diff());

        let mut current = 0;

        self.histories.iter_mut().for_each(|h| {
            while let Some(mut v) = h.differences.pop() {
                current += v.pop().unwrap().0;
            }
        });
        current
    }

    fn get_previous_value_sum(&mut self) -> isize {
        self.histories
            .iter_mut()
            .map(|h| {
                h.values.reverse();
                h
            })
            .for_each(|h| h.generate_diff());

        self.get_next_value_sum()
    }
}

fn part1(input: &str) -> isize {
    let mut sensor = input.parse::<Sensor>().unwrap();
    sensor.get_next_value_sum()
}

fn part2(input: &str) -> isize {
    let mut sensor = input.parse::<Sensor>().unwrap();
    sensor.get_previous_value_sum()
}

pub fn day() {
    let input = include_str!("../input/day9.txt");
    print!("Day 9\t");
    print!("Part 1: {}\t", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2);
    }
}
