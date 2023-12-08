use std::collections::HashMap;
use std::ops::Index;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct CalibrationValue {
    lines: Vec<String>,
}

type Position = usize;
type Number = usize;

impl CalibrationValue {
    fn new(input: &str) -> Self {
        CalibrationValue {
            lines: input.lines().map(|s| s.to_string()).collect(),
        }
    }

    fn part1(&mut self) -> usize {
        self.sum()
    }

    fn part2(&mut self) -> usize {
        for line in &mut self.lines {
            let mut found_numbers: HashMap<Position, Number> = HashMap::new();
            for (number_index, &number) in NUMBERS.iter().enumerate() {
                // could be more than one occurrence in a line
                for (position, _) in line.match_indices(number) {
                    found_numbers.insert(position, number_index + 1);
                }
            }

            if let Some(min_position) = found_numbers.iter().min_by_key(|(&position, _)| position) {
                line.replace_range(
                    min_position.0..&(min_position.0 + 1),
                    min_position.1.to_string().as_str(),
                );
            }

            if let Some(max_position) = found_numbers.iter().max_by_key(|(&position, _)| position) {
                line.replace_range(
                    max_position.0..&(max_position.0 + 1),
                    max_position.1.to_string().as_str(),
                );
            }
        }

        self.sum()
    }
}

impl Iterator for CalibrationValue {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.lines.pop() {
            let mut numbers = line.chars().filter(|c| c.is_digit(10)).collect::<String>();

            let mut result = String::new();
            result += &numbers.index(0..1).to_string();
            result += &numbers.pop().unwrap().to_string();
            result.parse().ok()
        } else {
            None
        }
    }
}

fn part1(input: &str) -> usize {
    let mut calibration_value = CalibrationValue::new(input);
    calibration_value.part1()
}

fn part2(input: &str) -> usize {
    let mut calibration_value = CalibrationValue::new(input);
    calibration_value.part2()
}

pub fn day() {
    let input = include_str!("../input/day1.txt");
    println!(
        "Day 1\t Part 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(part2(input), 281);
    }
}
