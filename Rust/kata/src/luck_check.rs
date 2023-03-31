struct LuckCheck {
    input: String,
}

impl TryFrom<&str> for LuckCheck {
    type Error = &'static str;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input == "" {
            return Err("Empty string");
        }

        input
            .chars()
            .all(|c| c.is_numeric())
            .then(|| LuckCheck {
                input: input.to_string(),
            })
            .ok_or("Input must be a string of digits")
    }
}

impl LuckCheck {
    fn is_lucky(&self) -> bool {
        println!("input: {}", self.input);
        let middle = self.input.len() / 2;
        let is_even = self.input.len() % 2 == 0;

        let iter = self.input.chars();
        let iter2 = iter.clone();

        let left = iter.take(middle);
        let right = iter2
            .skip(middle + is_even.then(|| 0).unwrap_or(1))
            .take(middle);

        left.chain(right)
            .map(|c| c.to_digit(10).unwrap())
            .map(|d| i32::try_from(d).expect("digit to i32"))
            .map(|d| {
                println!("d: {}", d);
                d
            })
            .enumerate()
            .fold(0, |acc, (i, n)| if i < middle { acc + n } else { acc - n })
            == 0
    }
}

// https://www.codewars.com/kata/5314b3c6bb244a48ab00076c
fn luck_check(ticket: &str) -> Option<bool> {
    LuckCheck::try_from(ticket).map(|lc| lc.is_lucky()).ok()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::luck_check;

    fn dotest(s: &str, expected: Option<bool>) {
        let actual = luck_check(s);
        assert!(
            actual == expected,
            "With ticket = \"{s}\"\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("683179", Some(true));
        dotest("683000", Some(false));
        dotest("6F43E8", None);
        dotest("", None);
        dotest("91856399083", Some(true));
        dotest("1233499943", Some(false));
    }
}
