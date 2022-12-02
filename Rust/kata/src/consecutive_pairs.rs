use itertools::Itertools;

trait Difference {
    type Output;
    /// Sort and add the difference between all pairs. Can fail, if len < 2.
    fn try_sum_difference(&self) -> Result<Self::Output, DifferenceError>;
}

impl Difference for [i8] {
    type Output = i8;
    fn try_sum_difference(&self) -> Result<Self::Output, DifferenceError> {
        if self.len() < 2 {
            return Err(DifferenceError::SliceTooShort);
        }
        Ok(self
            .iter()
            .sorted()
            .tuple_windows()
            .map(|w: (&i8, &i8)| w.1 - w.0)
            .sum())
    }
}

enum DifferenceError {
    SliceTooShort,
}

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    arr.try_sum_difference().ok()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_of_differences;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    #[test]
    fn sample_tests() {
        assert_eq!(sum_of_differences(&[1, 2, 10]), Some(9), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-3, -2, -1]), Some(2), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 1, 1, 1, 1]), Some(0), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-17, 17]), Some(34), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[0]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1]), None, "{}", ERR_MSG);
    }
}
