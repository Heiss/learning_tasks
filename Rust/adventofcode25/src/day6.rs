use std::fmt::Display;
use std::str::FromStr;

enum Operation {
    Add,
    Multiply,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
        }
    }
}

struct Worksheet {
    data: Vec<Vec<usize>>,
    operations: Vec<Operation>,
}

impl Display for Worksheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.data {
            for c in r {
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }
        for o in &self.operations {
            write!(f, "{} ", o)?;
        }
        Ok(())
    }
}

impl FromStr for Worksheet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        let mut operations = Vec::new();

        for l in s.lines() {
            let mut row = Vec::new();
            for c in l.split_whitespace() {
                if c == "*" {
                    operations.push(Operation::Multiply);
                } else if c == "+" {
                    operations.push(Operation::Add);
                } else {
                    if let Ok(v) = c.parse::<usize>() {
                        row.push(v);
                    }
                }
            }
            if !row.is_empty() {
                data.push(row);
            }
        }

        Ok(Worksheet { data, operations })
    }
}

impl Worksheet {
    fn get_column(&self, col: usize) -> Vec<usize> {
        self.data
            .iter()
            .map(|r| match r.get(col) {
                Some(v) => *v,
                None => match self.get_operation(col) {
                    Some(Operation::Add) => 0,
                    Some(Operation::Multiply) => 1,
                    None => panic!("No operation found"),
                },
            })
            .collect()
    }

    fn get_operation(&self, col: usize) -> Option<&Operation> {
        self.operations.get(col)
    }

    fn calculate_column(&self, col: usize) -> usize {
        match self.get_operation(col) {
            Some(Operation::Add) => self.get_column(col).iter().sum(),
            Some(Operation::Multiply) => self.get_column(col).iter().product(),
            None => self.get_column(col)[0],
        }
    }

    fn calculate_all_columns(&self) -> usize {
        (0..self.data[0].len())
            .map(|c| self.calculate_column(c))
            .map(|v| v)
            .sum()
    }
}

struct WorksheetText {
    data: String,
}

impl WorksheetText {
    fn new(data: &str) -> Self {
        WorksheetText {
            data: data.to_string(),
        }
    }

    fn get_operations_line(&self) -> &str {
        self.data.lines().last().unwrap()
    }

    fn find_nth_operation_idx(&self, n: usize) -> usize {
        if n == 0 {
            panic!("Invalid operation index");
        }
        let mut seen_operations = 0;
        for (idx, c) in self.get_operations_line().chars().enumerate() {
            if c == '*' || c == '+' {
                seen_operations += 1;
                if seen_operations == n {
                    return idx;
                }
            }
        }
        panic!("No operation found");
    }

    fn count_spaces_for_nth_operation(&self, n: usize) -> usize {
        if n == 0 {
            panic!("Invalid operation index");
        }
        let mut spaces = 1; //because the operation itself is counted as a space
        let idx = self.find_nth_operation_idx(n);
        let mut last_element = true;
        for c in self.get_operations_line().chars().skip(idx + 1) {
            if c == ' ' {
                spaces += 1;
            } else {
                last_element = false;
                break;
            }
        }
        if last_element {
            spaces += 1;
        }
        spaces
    }

    fn read_numbers_rtl(&self, col: usize) -> Vec<usize> {
        let mut numbers = Vec::new();
        let ops_idx = self.find_nth_operation_idx(col + 1);
        let spaces = self.count_spaces_for_nth_operation(col + 1);

        for idx in (0..spaces).rev() {
            let mut number = String::new();
            for l in self.data.lines().rev().skip(1) {
                if let Some(v) = l.get(ops_idx + idx..=ops_idx + idx) {
                    number.extend(v.chars().filter(|c| c.is_numeric()));
                }
            }

            if let Ok(v) = number.chars().rev().collect::<String>().parse::<usize>() {
                numbers.push(v);
            }
        }

        numbers
    }
}

impl From<WorksheetText> for Worksheet {
    fn from(value: WorksheetText) -> Self {
        let operations: Vec<Operation> = value
            .data
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|v| match v {
                "*" => Operation::Multiply,
                "+" => Operation::Add,
                _ => panic!("Invalid operation"),
            })
            .collect();
        let cols = value
            .data
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .count();
        let mut data: Vec<Vec<usize>> = (0..cols).map(|c| value.read_numbers_rtl(c)).collect();

        let max_len = data.iter().map(|r| r.len()).max().unwrap();
        for (i, row) in data.iter_mut().enumerate() {
            let default = match operations.get(i).unwrap() {
                Operation::Add => 0,
                Operation::Multiply => 1,
            };
            if row.len() < max_len {
                row.extend(std::iter::repeat(default).take(max_len - row.len()));
            }
        }
        let transposed_data = (0..max_len)
            .map(|i| data.iter().filter_map(|row| row.get(i).copied()).collect())
            .collect();

        let result = Worksheet {
            data: transposed_data,
            operations,
        };
        result
    }
}

fn part1(input: &str) -> usize {
    Worksheet::from_str(input).unwrap().calculate_all_columns()
}

fn part2(input: &str) -> usize {
    Worksheet::from(WorksheetText::new(input)).calculate_all_columns()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day6.txt");
    format!("Day 6\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + "#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 4277556);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 3263827);
    }
}
