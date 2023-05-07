use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug)]
enum SimpleOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<char> for SimpleOperator {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(SimpleOperator::Add),
            '-' => Ok(SimpleOperator::Sub),
            '*' => Ok(SimpleOperator::Mul),
            '/' => Ok(SimpleOperator::Div),
            _ => Err("Invalid operator"),
        }
    }
}

impl SimpleOperator {
    fn get_op<T>(self) -> impl Fn(T, T) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>,
    {
        match self {
            SimpleOperator::Add => |a, b| a + b,
            SimpleOperator::Sub => |a, b| a - b,
            SimpleOperator::Mul => |a, b| a * b,
            SimpleOperator::Div => |a, b| a / b,
        }
    }
}

#[derive(Debug)]
enum Token<T>
where
    T: FromStr,
{
    Number(T),
    Operator(SimpleOperator),
}

enum Operand<T>
where
    T: Add + Mul + Div + Sub,
{
    Value(T),
    Operator(SimpleOperator, Box<Operand<T>>, Box<Operand<T>>), // restriction: no unary operators
}

impl<T> From<Tokens<T>> for Operand<T>
where
    T: Add + Mul + Div + Sub + FromStr + Debug,
{
    fn from(mut tokens: Tokens<T>) -> Self {
        // easy kyu
        let mut tokens = tokens.0;
        assert_eq!(tokens.len(), 3);
        let left = tokens.remove(0);
        let op = tokens.remove(0);
        let right = tokens.remove(0);

        match (left, op, right) {
            (Token::Number(l), Token::Operator(o), Token::Number(r)) => {
                Operand::Operator(o, Box::new(Operand::Value(l)), Box::new(Operand::Value(r)))
            }
            _ => panic!("Invalid expression"),
        }
    }
}

impl<T> Operand<T>
where
    T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + FromStr,
{
    fn get_result(self) -> T {
        match self {
            Operand::Value(v) => v,
            Operand::Operator(o, l, r) => o.get_op()(l.get_result(), r.get_result()),
        }
    }
}

struct Tokens<T>(Vec<Token<T>>)
where
    T: FromStr;

impl<T> From<&str> for Tokens<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fn from(token_str: &str) -> Self {
        let mut tokens = Vec::new();
        let mut current_number = String::new();
        for c in token_str.chars() {
            if c.is_numeric() {
                current_number.push(c);
            } else {
                if !current_number.is_empty() {
                    tokens.push(Token::Number(
                        current_number.parse().expect("Cannot parse number"),
                    ));
                    current_number.clear();
                }
                tokens.push(Token::Operator(
                    SimpleOperator::try_from(c).expect("Cannot parse operator"),
                ));
            }
        }
        if !current_number.is_empty() {
            tokens.push(Token::Number(
                current_number
                    .parse()
                    .expect("Cannot parse number to finish"),
            ));
        }
        Tokens(tokens)
    }
}

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    let tokens = Tokens::from(format!("{}{}{}", value1, operator, value2).as_str());
    Operand::from(tokens).get_result()
}

// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::basic_op;

    fn dotest(op: char, v1: i32, v2: i32, expected: i32) {
        let actual = basic_op(op, v1, v2);
        assert_eq!(actual, expected, "With operator = '{op}', value1 = {v1}, value2 = {v2}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn example_tests() {
        dotest('+', 4, 7, 11);
        dotest('-', 15, 18, -3);
        dotest('*', 5, 5, 25);
        dotest('/', 49, 7, 7);
    }
}
