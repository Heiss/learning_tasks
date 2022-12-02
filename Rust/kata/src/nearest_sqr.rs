#[derive(Debug)]
enum SquareNumberError {
    NoSquareFound,
}

fn is_square(n: u32) -> bool {
    let root = (n as f64).sqrt();
    root == root.floor()
}

struct SquareNumber(u32);

impl SquareNumber {
    fn new(n: u32) -> Self {
        SquareNumber(n)
    }

    fn nearest_sqr(&self) -> Result<u32, SquareNumberError> {
        let mut i = 0;
        while self.0 - i != 0 {
            let (left, right) = (self.0 - i, self.0 + i);

            if is_square(left) {
                return Ok(left);
            }

            if is_square(right) {
                return Ok(right);
            }

            i += 1;
        }

        Err(SquareNumberError::NoSquareFound)
    }
}

fn nearest_sq(n: u32) -> u32 {
    SquareNumber::new(n).nearest_sqr().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::nearest_sq;

    #[test]
    fn sample_tests() {
        // assertion(expected, n)
        assertion(1, 1);
        assertion(1, 2);
        assertion(9, 10);
        assertion(121, 111);
        assertion(10000, 9999);
    }

    fn assertion(expected: u32, n: u32) {
        let actual = nearest_sq(n);
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n n: {}\n",
            expected,
            actual,
            n
        );
    }
}
