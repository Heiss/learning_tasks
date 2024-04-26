struct MinMax<'a, T> {
    min: &'a T,
    max: &'a T,
}

impl<'a, T> MinMax<'a, T>
where
    T: PartialOrd,
{
    pub fn new(left: &'a T, right: &'a T) -> Self {
        MinMax {
            min: left,
            max: right,
        }
    }

    pub fn add(&mut self, value: &'a T) {
        if value < self.min {
            self.min = value;
        } else if value > self.max {
            self.max = value;
        }
    }
}

impl<'a, T> TryFrom<&'a [T]> for MinMax<'a, T>
where
    T: PartialOrd,
{
    type Error = ();

    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        }

        let mut min_max = MinMax::new(&value[0], &value[0]);
        for v in value {
            min_max.add(v);
        }
        Ok(min_max)
    }
}

pub fn min_max(lst: &[i32]) -> (i32, i32) {
    let min_max = MinMax::try_from(lst).unwrap();
    (*min_max.min, *min_max.max)
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::min_max;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected: (i32, i32)) {
        assert_eq!(min_max(arr), expected, "{ERR_MSG} with lst = {arr:?}")
    }

    #[test]
    fn fixed_tests() {
        for (arr, expected) in [
            (vec![1, 2, 3, 4, 5], (1, 5)),
            (vec![2334454, 5], (5, 2334454)),
        ] {
            dotest(&arr, expected)
        }
    }
}
