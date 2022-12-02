use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};

pub fn rps(p1: &str, p2: &str) -> &'static str {
    match Game::new(
        p1.try_into().expect("Invalid Player 1 input"),
        p2.try_into().expect("Invalid Player 2 input"),
    )
    .exec()
    {
        Event::Win(Player(1)) => "Player 1 won!",
        Event::Win(Player(_)) => "Player 2 won!",
        Event::Draw => "Draw!",
    }
}

struct Player(u8);

enum Event {
    Win(Player),
    Draw,
}

#[derive(PartialEq)]
enum Choose {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum ChooseError {
    InvalidInput,
}

struct Game {
    p1: Choose,
    p2: Choose,
}

impl TryFrom<&str> for Choose {
    type Error = ChooseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "rock" => Ok(Choose::Rock),
            "paper" => Ok(Choose::Paper),
            "scissors" => Ok(Choose::Scissors),
            _ => Err(ChooseError::InvalidInput),
        }
    }
}

impl Choose {
    fn wins_over(&self) -> &[Choose] {
        match self {
            Choose::Rock => &[Choose::Scissors],
            Choose::Paper => &[Choose::Rock],
            Choose::Scissors => &[Choose::Paper],
        }
    }
}
impl PartialOrd for Choose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self == other).then(|| Equal).or_else(|| {
            self.wins_over()
                .contains(other)
                .then(|| Less)
                .or(Some(Greater))
        })
    }
}

impl Game {
    fn new(p1: Choose, p2: Choose) -> Self {
        Game { p1, p2 }
    }

    pub fn exec(&self) -> Event {
        match self.p1.partial_cmp(&self.p2).unwrap() {
            std::cmp::Ordering::Equal => Event::Draw,
            std::cmp::Ordering::Less => Event::Win(Player(1)),
            std::cmp::Ordering::Greater => Event::Win(Player(2)),
        }
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rps;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}
