/*
https://www.codewars.com/kata/5a4d303f880385399b000001/train/rust

DESCRIPTION:
Definition
Strong number is the number that the sum of the factorial of its digits is equal to number itself.

For example, 145 is strong, since 1! + 4! + 5! = 1 + 24 + 120 = 145.

Task
Given a number, Find if it is Strong or not and return either "STRONG!!!!" or "Not Strong !!".

Notes
Number passed is always Positive.
Return the result as String
Input >> Output Examples
strong_num(1) ==> return "STRONG!!!!"
Since, the sum of its digits' factorial (1) is equal to number itself, then its a Strong.

strong_num(123) ==> return "Not Strong !!"
Since the sum of its digits' factorial of 1! + 2! + 3! = 9 is not equal to number itself, then it's Not Strong .

strong_num(2)  ==>  return "STRONG!!!!"
Since the sum of its digits' factorial of 2! = 2 is equal to number itself, then its a Strong.

strong_num(150) ==> return "Not Strong !!"
Since the sum of its digits' factorial of 1! + 5! + 0! = 122 is not equal to number itself, Then it's Not Strong .
*/

fn strong(n: u64) -> String {
    StrongNumber::new(n)
        .is_strong()
        .then(|| "STRONG!!!!")
        .unwrap_or("Not Strong !!")
        .to_string()
}

struct StrongNumber(u64, Vec<u64>);

fn fac(val: &u64) -> u64 {
    (1..val + 1).product()
}
impl StrongNumber {
    fn new(val: u64) -> Self {
        StrongNumber(
            val,
            val.to_string()
                .chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect(),
        )
    }
    fn is_strong(&self) -> bool {
        self.0 == self.calculate()
    }

    fn calculate(&self) -> u64 {
        self.1.iter().map(fac).sum()
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");

        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");

        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");

        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");

        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");

        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");
    }
}
