use itertools::Itertools;
use std::convert::TryFrom;
use std::mem;

#[derive(Debug)]
struct Calories(u64);
impl Calories {
    fn calories(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
enum CaloriesError {
    NotANumber,
}
impl TryFrom<&str> for Calories {
    type Error = CaloriesError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<u64>() {
            Ok(n) => Ok(Calories(n)),
            Err(_) => Err(CaloriesError::NotANumber),
        }
    }
}

#[derive(Debug)]
struct Elf {
    calories: Vec<Calories>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new(),
        }
    }

    fn add_calories(&mut self, calories: Calories) {
        self.calories.push(calories);
    }

    fn calories(&self) -> u64 {
        self.calories.iter().map(|c| c.calories()).sum()
    }
}

#[derive(Debug)]
enum ElvesBuilderError {
    NotValidNumberFound,
}

#[derive(Debug)]
struct ElvesBuilder {
    elves: Vec<Elf>,
}

impl ElvesBuilder {
    fn finish(self) -> Vec<Elf> {
        self.elves
    }
}

impl TryFrom<&str> for ElvesBuilder {
    type Error = ElvesBuilderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut elves = Vec::new();
        let mut elf: Option<Elf> = None;
        for line in value.lines() {
            if line == "" {
                if let Some(e) = mem::take(&mut elf) {
                    elves.push(e);
                }
            } else {
                let Ok(calories) = Calories::try_from(line) else {
                     return Err(ElvesBuilderError::NotValidNumberFound);
                };

                if let Some(e) = &mut elf {
                    e.add_calories(calories);
                } else {
                    let mut temp_elf = Elf::new();
                    temp_elf.add_calories(calories);
                    elf = Some(temp_elf);
                }
            }
        }

        if let Some(e) = elf {
            elves.push(e);
        }

        Ok(ElvesBuilder { elves })
    }
}

fn day1_part1(text: &str) -> u64 {
    ElvesBuilder::try_from(text)
        .expect("Not valid input")
        .finish()
        .iter()
        .map(|e| e.calories())
        .max()
        .unwrap()
}

fn day1_part2(text: &str) -> u64 {
    ElvesBuilder::try_from(text)
        .expect("Not valid input")
        .finish()
        .into_iter()
        .sorted_unstable_by_key(|e| e.calories())
        .rev()
        .take(3)
        .map(|e| e.calories())
        .sum()
}

pub fn day1(text: &str) {
    println!("Day1 part 1: {}", day1_part1(text));
    println!("Day1 part 2: {}", day1_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day1_part1, day1_part2};

    #[test]
    fn fixed_tests() {
        let test_input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        assert_eq!(day1_part1(test_input), 24000);
        assert_eq!(day1_part2(test_input), 45000);
    }
}
