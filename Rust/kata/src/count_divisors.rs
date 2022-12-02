fn divisors(n: u32) -> u32 {
    (1..=n).filter(|&i| n % i == 0).count() as u32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::divisors;

    #[test]
    fn sample_tests() {
        assert_eq!(divisors(1), 1);
        assert_eq!(divisors(4), 3);
        assert_eq!(divisors(5), 2);
        assert_eq!(divisors(12), 6);
        assert_eq!(divisors(25), 3);
        assert_eq!(divisors(4096), 13);
    }
}
