use std::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Choose {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
enum ChooseError {
    NotValidInput,
}

#[derive(Debug)]
enum Event {
    Win,
    Lose,
    Draw,
}

impl Event {
    fn get_value(&self) -> u64 {
        match self {
            Event::Lose => 0,
            Event::Draw => 3,
            Event::Win => 6,
        }
    }
}

impl Choose {
    fn get_value(&self) -> u64 {
        match self {
            Choose::Rock => 1,
            Choose::Paper => 2,
            Choose::Scissor => 3,
        }
    }

    fn get_win(&self) -> Choose {
        match self {
            Choose::Rock => Choose::Paper,
            Choose::Scissor => Choose::Rock,
            Choose::Paper => Choose::Scissor,
        }
    }

    fn get_lose(&self) -> Choose {
        match self {
            Choose::Rock => Choose::Scissor,
            Choose::Scissor => Choose::Paper,
            Choose::Paper => Choose::Rock,
        }
    }

    fn cmp(&self, other: &Choose) -> Event {
        if self == other {
            Event::Draw
        } else if &other.get_win() == self {
            Event::Win
        } else {
            Event::Lose
        }
    }
}

impl TryFrom<&str> for Choose {
    type Error = ChooseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Choose::Rock),
            "B" | "Y" => Ok(Choose::Paper),
            "C" | "Z" => Ok(Choose::Scissor),
            _ => Err(ChooseError::NotValidInput),
        }
    }
}

struct Game<F> {
    rules: F,
    score: u64,
}

impl<F> Game<F>
where
    F: Fn(Choose, Choose) -> u64,
{
    fn new(rules: F) -> Game<F> {
        Game { rules, score: 0 }
    }

    fn play(&mut self, input: &str) -> &mut Self {
        for line in input.lines() {
            let mut iter = line.split_whitespace();

            while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
                let a = Choose::try_from(a).expect("Not valid input");
                let b = Choose::try_from(b).expect("Not valid input");
                let score = (self.rules)(a, b);
                self.score += score;
            }
        }

        self
    }
}

fn rule1(a: Choose, b: Choose) -> u64 {
    b.get_value() + b.cmp(&a).get_value()
}

fn rule2(a: Choose, b: Choose) -> u64 {
    let b = match b {
        Choose::Rock => a.get_lose(),
        Choose::Paper => a,
        Choose::Scissor => a.get_win(),
    };

    rule1(a, b)
}

fn day2_part1(text: &str) -> u64 {
    Game::new(rule1).play(text).score
}

fn day2_part2(text: &str) -> u64 {
    Game::new(rule2).play(text).score
}

pub fn day2(text: &str) {
    println!("Day 2 part 1: {}", day2_part1(text));
    println!("Day 2 part 2: {}", day2_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day2_part1, day2_part2};

    #[test]
    fn fixed_tests() {
        let test_input = r#"A Y
B X
C Z"#;

        assert_eq!(day2_part1(test_input), 15);
        assert_eq!(day2_part2(test_input), 12);
    }
}
