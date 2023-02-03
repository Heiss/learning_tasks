enum ParanError {
    InvalidChar,
    NotPossible,
}

struct Paran {
    text: String,
}

impl TryFrom<&str> for Paran {
    type Error = ParanError;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        text.chars()
            .all(|c| c == '(' || c == ')')
            .then(|| Paran {
                text: text.to_string(),
            })
            .ok_or(ParanError::InvalidChar)
    }
}

impl Paran {
    fn reduce(&mut self) {
        let mut stack = Vec::new();
        for c in self.text.chars() {
            match c {
                '(' => stack.push(c),
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                _ => unreachable!(),
            }
        }
        self.text = stack.into_iter().collect();
    }

    fn get_reverse_count(mut self) -> Result<usize, ParanError> {
        self.reduce();
        if self.text.len() % 2 != 0 {
            return Err(ParanError::NotPossible);
        }

        if self.text.is_empty() {
            return Ok(0);
        }

        let mut count = 0;
        if &self.text[0..2] == "))" {
            count += 1;
        } else if &self.text[0..2] == "((" {
            count += 1;
        } else if &self.text[0..2] == ")(" {
            count += 2;
        };
        let reduced = &self.text[2..];

        Ok(count + Paran::try_from(reduced)?.get_reverse_count()?)
    }
}

fn solve(s: &str) -> Option<usize> {
    let mut paran = Paran::try_from(s).ok()?;
    paran.get_reverse_count().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("((()"), Some(1));
        assert_eq!(solve("((("), None);
        assert_eq!(solve("())((("), Some(3));
        assert_eq!(solve("())()))))()()("), Some(4));
        assert_eq!(solve(")()("), Some(2));
    }
}
