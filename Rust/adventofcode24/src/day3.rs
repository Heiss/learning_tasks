use std::cmp::PartialEq;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum LexItem {
    Num(usize),
    ParanLeft,
    ParanRight,
    Unspecified,
    Komma,
    Mul,
    Do,
    Dont,
}

#[derive(Debug)]
enum Token {
    Mul(usize, usize),
    Do,
    Dont,
}

impl Token {
    fn calculate(&self) -> usize {
        match self {
            Token::Mul(a, b) => a * b,
            _ => 0,
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
}

impl FromStr for Parser {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars().peekable();
        let mut lexes = Vec::new();

        while let Some(&c) = it.peek() {
            match c {
                'm' => {
                    it.next();
                    if Some(&'u') != it.peek() {
                        continue;
                    }
                    it.next();
                    if Some(&'l') != it.peek() {
                        continue;
                    }

                    lexes.push(LexItem::Mul);
                    it.next();
                }
                'd' => {
                    it.next();
                    if Some(&'o') != it.peek() {
                        continue;
                    }
                    it.next();
                    if Some(&'n') == it.peek() {
                        it.next();
                        if Some(&'\'') != it.peek() {
                            continue;
                        }

                        it.next();
                        if Some(&'t') != it.peek() {
                            continue;
                        }

                        lexes.push(LexItem::Dont)
                    } else {
                        lexes.push(LexItem::Do)
                    }
                    it.next();
                }
                '0'..='9' => {
                    let n = {
                        let mut number = String::new();
                        while let Some(&c) = it.peek() {
                            if !c.is_numeric() {
                                break;
                            }

                            number.push(c);
                            it.next();
                        }
                        number.parse::<usize>().unwrap()
                    };
                    lexes.push(LexItem::Num(n));
                }
                '(' => {
                    it.next();
                    lexes.push(LexItem::ParanLeft);
                }
                ',' => {
                    it.next();
                    lexes.push(LexItem::Komma);
                }
                ')' => {
                    it.next();
                    lexes.push(LexItem::ParanRight);
                }
                _ => {
                    it.next();
                    lexes.push(LexItem::Unspecified);
                }
            }
        }

        // drop mutable
        let lexes = lexes;

        let mut tokens = Vec::new();
        let mut it = lexes.iter().peekable();

        while let Some(&v) = it.peek() {
            match v {
                LexItem::Do => tokens.push(Token::Do),
                LexItem::Dont => tokens.push(Token::Dont),
                LexItem::Mul => {
                    it.next();
                    if Some(&&LexItem::ParanLeft) != it.peek() {
                        continue;
                    }
                    it.next();
                    let n1;
                    if let Some(&&LexItem::Num(n)) = it.peek() {
                        n1 = n;
                    } else {
                        continue;
                    }
                    it.next();
                    if Some(&&LexItem::Komma) != it.peek() {
                        continue;
                    }
                    it.next();
                    let n2;
                    if let Some(&&LexItem::Num(n)) = it.peek() {
                        n2 = n;
                    } else {
                        continue;
                    }
                    it.next();
                    if Some(&&LexItem::ParanRight) != it.peek() {
                        continue;
                    }
                    tokens.push(Token::Mul(n1, n2));
                }
                _ => {}
            }
            it.next();
        }

        Ok(Self { tokens })
    }
}

impl Parser {
    fn calculate(&self) -> usize {
        self.tokens.iter().map(Token::calculate).sum()
    }

    fn calculate_precisely(&self) -> usize {
        let mut enabled = true;
        let mut result = 0;

        for t in self.tokens.iter() {
            match t {
                Token::Do => enabled = true,
                Token::Dont => enabled = false,
                v => {
                    if enabled {
                        result += v.calculate()
                    }
                }
            }
        }

        result
    }
}

fn part1(input: &str) -> usize {
    Parser::from_str(input).unwrap().calculate()
}

fn part2(input: &str) -> usize {
    Parser::from_str(input).unwrap().calculate_precisely()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day3.txt");
    format!("Day 3\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const INPUT: &str =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(part1(INPUT), 161);
    }
    #[test]
    fn day2() {
        const INPUT: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(part2(INPUT), 48);
    }
}
