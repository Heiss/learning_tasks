use itertools::Itertools;
use std::iter::Skip;
use std::str::Split;

#[derive(PartialEq, Debug)]
struct Symbol<'a>(&'a str);

#[derive(PartialEq, Debug)]
struct Crate<'a> {
    symbol: Symbol<'a>,
}

impl<'a> TryFrom<&'a str> for Crate<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        (value != " " && value.to_uppercase().as_str() == value)
            .then(|| Self {
                symbol: Symbol(value),
            })
            .ok_or("Invalid crate symbol")
    }
}

struct Steps {
    from: usize,
    to: usize,
    count: usize,
}

impl TryFrom<&str> for Steps {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace();
        iter.next().ok_or("move not found")?;
        let count = iter
            .next()
            .ok_or("Invalid count")?
            .parse::<usize>()
            .expect("Invalid count value");
        iter.next().ok_or("from not found")?;
        let from = iter
            .next()
            .ok_or("Invalid from")?
            .parse::<usize>()
            .expect("Invalid from value")
            - 1;
        iter.next().ok_or("to not found")?;
        let to = iter
            .next()
            .ok_or("Invalid range")?
            .parse::<usize>()
            .expect("Invalid to value")
            - 1;
        Ok(Self { from, to, count })
    }
}

struct CrateBuilderIterator<'a> {
    iter: Skip<Split<'a, &'a str>>,
    symbols_found: usize,
    symbols_count: usize,
}

impl<'a> CrateBuilderIterator<'a> {
    fn new(haystack: &'a str, symbols_count: usize) -> Self {
        Self {
            iter: haystack.split("").skip(1),
            symbols_found: 0,
            symbols_count,
        }
    }

    fn done(&self) -> bool {
        self.symbols_found == self.symbols_count
    }
}

#[derive(PartialEq, Debug)]
enum CrateBuilderResult<'a> {
    Found(Crate<'a>),
    NotFound,
}

/// This iterator will return a `Crate` if the next word in the haystack is a valid crate symbol.
/// Otherwise it will return `None`.
///
/// Beware: None does not mean, there is nothing left in the haystack. There could be an empty symbol in this place. Ask done() to be sure.
impl<'a> Iterator for CrateBuilderIterator<'a> {
    type Item = CrateBuilderResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done() {
            return None;
        }

        self.iter.next(); // skips left paren
        let crate_symbol = self.iter.next()?;
        self.iter.next(); // skips right paren
        self.iter.next(); // skips any whitespace

        if let Ok(crate_) = Crate::try_from(crate_symbol) {
            self.symbols_found += 1;
            Some(CrateBuilderResult::Found(crate_))
        } else {
            self.symbols_found += 1;
            Some(CrateBuilderResult::NotFound)
        }
    }
}

struct Crane {
    steps: Vec<Steps>,
}

impl TryFrom<&str> for Crane {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            steps: value
                .lines()
                .map(Steps::try_from)
                .filter_map(Result::ok)
                .collect(),
        })
    }
}

struct Ship<'a> {
    staple_of_crates: Vec<Vec<Crate<'a>>>,
    crane: Crane,
}

impl<'a> Ship<'a> {
    fn execute_9000(&mut self) {
        for step in self.crane.steps.iter() {
            for _i in 0..step.count {
                if let Some(crate_) = self.staple_of_crates[step.from].pop() {
                    self.staple_of_crates[step.to].push(crate_);
                }
            }
        }
    }

    fn execute_9001(&mut self) {
        for step in self.crane.steps.iter() {
            let mut holding = Vec::new();
            for _i in 0..step.count {
                if let Some(crate_) = self.staple_of_crates[step.from].pop() {
                    holding.push(crate_);
                }
            }
            while let Some(crate_) = holding.pop() {
                self.staple_of_crates[step.to].push(crate_);
            }
        }
    }

    fn get_top_crates(&self) -> String {
        self.staple_of_crates
            .iter()
            .map(|staple| staple.last())
            .filter_map(|symbol| symbol)
            .map(|crate_| crate_.symbol.0)
            .join("")
    }
}

impl<'a> TryFrom<&'a str> for Ship<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // take all lines with crate data until there is an empty line, the rest are move instructions
        let crates_iter = value.lines().skip_while(|&v| !v.is_empty()).skip(1);
        let mut crates_iter_backworts = value.lines().rev().skip_while(|&v| !v.is_empty()).skip(1);

        let count_of_crates = crates_iter_backworts
            .next()
            .ok_or("Invalid count of crates")?
            .split("   ")
            .count();
        let mut crates: Vec<Vec<Crate>> = Vec::with_capacity(count_of_crates);
        for _ in 0..count_of_crates {
            crates.push(Vec::new());
        }

        while let Some(line) = crates_iter_backworts.next() {
            let mut column = 0;
            let mut crate_builder = CrateBuilderIterator::new(line, count_of_crates);
            while let Some(crate_builder_result) = crate_builder.next() {
                match crate_builder_result {
                    CrateBuilderResult::Found(c) => crates[column].push(c),
                    CrateBuilderResult::NotFound => (),
                }
                column += 1;
            }
        }

        let crane = Crane::try_from(crates_iter.collect::<Vec<&str>>().join("\n").as_str())?;

        Ok(Self {
            staple_of_crates: crates,
            crane,
        })
    }
}

fn day5_part1(text: &str) -> String {
    let mut ship = Ship::try_from(text).expect("Invalid input");
    ship.execute_9000();
    ship.get_top_crates()
}

fn day5_part2(text: &str) -> String {
    let mut ship = Ship::try_from(text).expect("Invalid input");
    ship.execute_9001();
    ship.get_top_crates()
}

pub fn day5(text: &str) {
    println!("Day5 part 1: {}", day5_part1(text));
    println!("Day5 part 2: {}", day5_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day5_part1, day5_part2, Crate, CrateBuilderIterator, CrateBuilderResult, Symbol};

    const TEST_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day5_part1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day5_part2(TEST_INPUT), "MCD");
    }

    #[test]
    fn test_cratebuilder() {
        let mut builder = CrateBuilderIterator::new("[A] [B]", 2);
        assert_eq!(
            builder.next(),
            Some(CrateBuilderResult::Found(Crate {
                symbol: Symbol("A")
            }))
        );
        assert_eq!(
            builder.next(),
            Some(CrateBuilderResult::Found(Crate {
                symbol: Symbol("B")
            }))
        );
    }
}
