// https://www.codewars.com/kata/5277c8a221e209d3f6000b56/train/rust

enum Orientatation {
    Left(Braces),
    Right(Braces),
}

#[derive(PartialEq)]
enum Braces {
    Paran,
    Curly,
    Brackets,
}

enum ConversionError {
    InvalidValue,
}

impl TryFrom<char> for Orientatation {
    type Error = ConversionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Orientatation::Left(Braces::Paran)),
            ')' => Ok(Orientatation::Right(Braces::Paran)),
            '[' => Ok(Orientatation::Left(Braces::Brackets)),
            ']' => Ok(Orientatation::Right(Braces::Brackets)),
            '{' => Ok(Orientatation::Left(Braces::Curly)),
            '}' => Ok(Orientatation::Right(Braces::Curly)),
            _ => Err(ConversionError::InvalidValue),
        }
    }
}

fn valid_braces(s: &str) -> bool {
    let mut seen = Vec::new();

    s.chars()
        .map(|c| {
            let brace: Orientatation = match c.try_into() {
                Ok(v) => v,
                Err(_) => return false,
            };

            match brace {
                Orientatation::Left(v) => {
                    seen.push(v);
                    true
                }
                Orientatation::Right(v) => match seen.pop() {
                    Some(last) => last == v,
                    None => false,
                },
            }
        })
        .all(|v| v)
        && seen.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
    }

    fn expect_true(s: &str) {
        assert!(
            valid_braces(s),
            "Expected {s:?} to be valid. Got false",
            s = s
        );
    }

    fn expect_false(s: &str) {
        assert!(
            !valid_braces(s),
            "Expected {s:?} to be invalid. Got true",
            s = s
        );
    }
}
