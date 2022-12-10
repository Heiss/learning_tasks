use std::convert::TryFrom;
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Noop,
    Addx(isize),
}

impl Operation {
    fn get_cycle(&self) -> u8 {
        match self {
            Operation::Noop => 1,
            Operation::Addx(_) => 2,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            return Ok(Operation::Noop);
        }
        if value.starts_with("addx ") {
            let x = value[5..].parse::<isize>().map_err(|_| "invalid addx")?;
            return Ok(Operation::Addx(x));
        }
        Err("invalid operation")?
    }
}
struct Operations {
    operations: Vec<Operation>,
    cycle: isize,
    register_x: isize,
    sleep: u8,
    cache: Operation,
}

impl Operations {
    fn get_signal_samples(&mut self, samples: Vec<isize>) -> Vec<isize> {
        self.filter(|v| samples.contains(&v.0))
            .map(|v| v.0 * v.1)
            .collect()
    }
}

/// The signal holds the values for: (cycle, register_x, Operation).
type Signal = (isize, isize, Operation);

impl Iterator for Operations {
    type Item = Signal;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;

        if self.sleep > 0 {
            self.sleep -= 1;
            return Some((self.cycle, self.register_x, self.cache));
        }

        self.register_x += match self.cache {
            Operation::Addx(x) => x,
            _ => 0,
        };
        self.cache = Operation::Noop;

        if self.operations.is_empty() {
            return None;
        }

        self.cache = self.operations.remove(0);
        self.sleep = self.cache.get_cycle() - 1;
        Some((self.cycle, self.register_x, self.cache))
    }
}

impl TryFrom<&str> for Operations {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut operations = Vec::new();
        for line in value.lines() {
            let operation = Operation::try_from(line)?;
            operations.push(operation);
        }

        Ok(Operations {
            operations,
            cycle: 0,
            register_x: 1,
            sleep: 0,
            cache: Operation::Noop,
        })
    }
}

fn day10_part1(text: &str) -> isize {
    let samples = vec![20, 60, 100, 140, 180, 220];
    Operations::try_from(text)
        .unwrap()
        .get_signal_samples(samples)
        .iter()
        .sum()
}

fn day10_part2(text: &str) -> String {
    Operations::try_from(text)
        .unwrap()
        .map(|v| {
            let mut f = String::new();
            let position = (v.0 - 1) % 40;
            if position == 0 {
                write!(f, "\n").unwrap();
            }
            if vec![v.1 - 1, v.1, v.1 + 1].contains(&position) {
                write!(&mut f, "#").unwrap();
            } else {
                write!(&mut f, ".").unwrap();
            }
            f
        })
        .collect()
}

pub fn day10(text: &str) {
    println!("day10 part 1: {}", day10_part1(text));
    println!("day10 part 2: {}", day10_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day10_part1, day10_part2};
    use crate::day10::{Operation, Operations};

    const TEST_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day10_part1(TEST_INPUT), 13140);
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day10_part2(TEST_INPUT), 0);
    }
}
