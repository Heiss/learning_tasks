// https://www.codewars.com/kata/58f8b35fda19c0c79400020f

trait NonConsecutive {
    fn all_non_consecutive(&self) -> Vec<(usize, i32)>;
}

impl NonConsecutive for [i32] {
    fn all_non_consecutive(&self) -> Vec<(usize, i32)> {
        self.windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] + 1 != w[1])
            .map(|(i, w)| (i + 1, w[1]))
            .collect()
    }
}

fn all_non_consecutive(arr: &[i32]) -> Vec<(usize, i32)> {
    arr.all_non_consecutive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let expect: Vec<(usize, i32)> = vec![(4, 6), (7, 10)];
        let result = all_non_consecutive(&[1, 2, 3, 4, 6, 7, 8, 10]);

        assert_eq!(expect, result);
    }
}
