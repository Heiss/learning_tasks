use std::convert::TryFrom;

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
    fn get_op(self) -> impl Fn(i32, i32) -> i32 {
        match self {
            SimpleOperator::Add => |a, b| a + b,
            SimpleOperator::Sub => |a, b| a - b,
            SimpleOperator::Mul => |a, b| a * b,
            SimpleOperator::Div => |a, b| a / b,
        }
    }
}

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    SimpleOperator::try_from(operator)
        .expect("Not a valid input provided")
        .get_op()(value1, value2)
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
