use std::collections::VecDeque;
use std::str::FromStr;

struct Safe {
    current: usize,
    counter_zero_reached: usize,
    counter_any_zero_occured: usize,
    counter_invalid_zeroes: usize,
    size: usize,
}

#[derive(Debug)]
enum Error {
    Size,
    NotKnownRotation,
    InvalidRotationSize,
}

#[derive(Debug)]
enum Rotation {
    Right(usize),
    Left(usize),
}

impl Rotation {
    fn uniform(&self, size: usize) -> (usize, usize) {
        match self {
            Rotation::Right(n) => (n % size, n / size),
            Rotation::Left(n) => {
                let overflow = n / size;
                let mut num = n % size;
                num = size - num;
                (num, overflow)
            }
        }
    }
}

struct Rotations(Vec<Rotation>);
impl FromStr for Rotations {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rotations(
            s.lines()
                .map(|l| Rotation::from_str(l))
                .collect::<Result<Vec<Rotation>, Self::Err>>()?,
        ))
    }
}

impl FromStr for Rotation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();

        let rot = iter.next();
        let num: usize = iter
            .as_str()
            .parse()
            .map_err(|_| Error::InvalidRotationSize)?;

        match rot {
            Some('R') => Ok(Rotation::Right(num)),
            Some('L') => Ok(Rotation::Left(num)),
            _ => Err(Error::NotKnownRotation),
        }
    }
}

impl Safe {
    fn create(size: usize, current: usize) -> Result<Safe, Error> {
        if size < current {
            return Err(Error::Size);
        }

        Ok(Safe {
            current,
            counter_zero_reached: 0,
            counter_any_zero_occured: 0,
            counter_invalid_zeroes: 0,
            size: size + 1,
        })
    }

    fn do_rotations(&mut self, rotations: Vec<Rotation>) {
        for rot in rotations {
            let (num, overflow) = rot.uniform(self.size);
            self.counter_any_zero_occured += overflow;

            if let Rotation::Left(n) = rot {
                if num <= self.current {
                    self.counter_any_zero_occured += 1;
                } else {
                    self.counter_invalid_zeroes += 1;
                }
            }

            self.current += num;
            self.counter_any_zero_occured += self.current / self.size;
            self.current %= self.size;

            if self.current == 0 {
                self.counter_zero_reached += 1;
            }
        }
    }

    fn do_rotations_dumb(&mut self, rotations: Vec<Rotation>) {
        let mut rotations = VecDeque::from(rotations);
        while let Some(current_rot) = rotations.pop_front() {
            match current_rot {
                Rotation::Left(n) => {
                    for _ in 0..n {
                        if self.current == 0 {
                            self.current += self.size;
                        }
                        self.current -= 1;
                        if self.current == 0 {
                            self.counter_any_zero_occured += 1;
                        }
                    }
                }
                Rotation::Right(n) => {
                    for _ in 0..n {
                        self.current = (self.current + 1) % self.size;
                        if self.current == 0 {
                            self.counter_any_zero_occured += 1;
                        }
                    }
                }
            }
        }
    }

    fn get_counter_zero_reached(&self) -> usize {
        self.counter_zero_reached
    }

    fn get_counter_zero_occured(&self) -> usize {
        self.counter_any_zero_occured - self.counter_invalid_zeroes
    }
}

fn part1(input: &str) -> usize {
    let rotations: Rotations = input.parse().unwrap();
    let mut safe = Safe::create(99, 50).unwrap();
    safe.do_rotations(rotations.0);
    safe.get_counter_zero_reached()
}

fn part2(input: &str) -> usize {
    let rotations: Rotations = input.parse().unwrap();
    let mut safe = Safe::create(99, 50).unwrap();
    safe.do_rotations_dumb(rotations.0);
    safe.get_counter_zero_occured()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day1.txt");
    format!("Day 1\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(part1(INPUT), 3);
    }
    #[test]
    fn day2() {
        const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(part2(INPUT), 6);
    }
}
