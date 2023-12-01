struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut reversed = 0;
        struct Solution;

        let mut y = x;
        while y != 0 {
            reversed = reversed * 10 + y % 10;
            y /= 10;
        }

        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
