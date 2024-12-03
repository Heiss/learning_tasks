use std::str::FromStr;

#[derive(Debug)]
enum LexItem {
    Char(char),
    Num(usize),
    ParanLeft,
    ParanRight,
    Unspecified,
    Komma,
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
                'm' | 'u' | 'l' | 'd' | 'o' | 'n' | '\'' | 't' => {
                    it.next();
                    lexes.push(LexItem::Char(c))
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
                v => {
                    it.next();
                    lexes.push(LexItem::Unspecified);
                }
            }
        }

        // drop mutable
        let lexes = lexes;

        let mut tokens = Vec::new();
        for v in lexes.windows(8) {
            match v {
                [LexItem::Char('m'), LexItem::Char('u'), LexItem::Char('l'), LexItem::ParanLeft, LexItem::Num(n1), LexItem::Komma, LexItem::Num(n2), LexItem::ParanRight] => {
                    tokens.push(Token::Mul(*n1, *n2))
                }
                [LexItem::Char('d'), LexItem::Char('o'), LexItem::Char('n'), LexItem::Char('\''), LexItem::Char('t'), LexItem::ParanLeft, LexItem::ParanRight, ..] => {
                    tokens.push(Token::Dont)
                }
                [LexItem::Char('d'), LexItem::Char('o'), LexItem::ParanLeft, LexItem::ParanRight, ..] => {
                    tokens.push(Token::Do)
                }
                _ => {}
            }
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
    let input = include_str!("../input/day3.txt");
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
