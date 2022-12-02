/*
https://www.codewars.com/kata/55f2b110f61eb01779000053/train/rust

Given two integers a and b, which can be positive or negative, find the sum of all the integers between and including them and return it. If the two numbers are equal return a or b.

Note: a and b are not ordered!

Examples (a, b) --> output (explanation)
(1, 0) --> 1 (1 + 0 = 1)
(1, 2) --> 3 (1 + 2 = 3)
(0, 1) --> 1 (0 + 1 = 1)
(1, 1) --> 1 (1 since both are same)
(-1, 0) --> -1 (-1 + 0 = -1)
(-1, 2) --> 2 (-1 + 0 + 1 + 2 = 2)
*/

pub fn get_sum(a: i64, b: i64) -> i64 {
    SumHelper::new(a, b).exec()
}

struct SumHelper {
    min: i64,
    max: i64,
}

impl SumHelper {
    fn new(a: i64, b: i64) -> Self {
        let para = (a < b).then(|| (a, b)).unwrap_or((b, a));

        Self {
            min: para.0,
            max: para.1,
        }
    }

    fn exec(&self) -> i64 {
        (self.min..self.max + 1).sum()
    }
}

// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
