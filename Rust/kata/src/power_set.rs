use std::collections::HashSet;

// https://www.codewars.com/kata/53d3173cf4eb7605c10001a8
struct PowerSet {
    set: HashSet<Vec<u32>>,
}

impl PowerSet {
    fn new(set: &[u32]) -> Self {
        let mut power_set = HashSet::new();
        // 1 << set.len() ist die Anzahl der Elemente der zu berechnenden Potenzmenge
        // 1 << set.len() == 2^set.len() == #P(set)
        // Gleichzeitig stellt i die Binärzahl dar, die die Elemente der Potenzmenge repräsentiert.
        // z.B. 5 == 0b101 entspricht der Potenzmenge {set[0], set[2]}
        //      7 == 0b111 entspricht der Potenzmenge {set[0], set[1], set[2]}
        //
        // j ist der Index des Elements in der Menge set.
        // Wenn i & (1 << j) != 0, dann soll das j-te Element in der Potenzmenge enthalten sein.
        // Das ist der Fall, wenn das j-te Bit von i gesetzt ist.
        for i in 0..(1 << set.len()) {
            let mut subset = Vec::new();
            for j in 0..set.len() {
                if i & (1 << j) != 0 {
                    subset.push(set[j]);
                }
            }
            power_set.insert(subset);
        }
        Self { set: power_set }
    }
}

impl From<PowerSet> for Vec<Vec<u32>> {
    fn from(power_set: PowerSet) -> Self {
        power_set.set.into_iter().collect()
    }
}

fn power(a: &[u32]) -> Vec<Vec<u32>> {
    PowerSet::new(a).into()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::power;
    use core::iter::zip;
    use itertools::Itertools;
    use std::cmp::Ordering;

    fn f(a: &[u32], b: &[u32]) -> Ordering {
        for (x, y) in zip(a, b) {
            if x != y {
                return x.cmp(&y);
            }
        }
        return a.len().cmp(&b.len());
    }

    fn dotest(a: &[u32], expected: &[Vec<u32>]) {
        let actual = power(a);
        assert!(
            actual
                .iter()
                .map(|x| x.iter().sorted().copied().collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .iter()
                .sorted_by(|a, b| f(&a, &b))
                .eq(expected
                    .iter()
                    .map(|x| x.iter().sorted().copied().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
                    .iter()
                    .sorted_by(|a, b| f(a, b))),
            "With a = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[1, 2, 3],
            &[
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
        );
    }
}
