// https://www.codewars.com/kata/57ae18c6e298a7a6d5000c7a

use std::borrow::Cow;

fn replace_all<T: PartialEq + Copy>(xs: &[T], find: T, replace: T) -> Vec<T> {
    xs.iter()
        .map(|&x| if x == find { replace } else { x })
        .collect()
}

fn _replace_all_lazy<T: PartialEq + Copy>(xs: &[T], find: T, replace: T) -> Vec<Cow<T>> {
    xs.iter()
        .map(|x| {
            if *x == find {
                Cow::Owned(replace)
            } else {
                Cow::Borrowed(x)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn basic() {
        assert_eq!(replace_all(&[], 1, 2), []);
        assert_eq!(replace_all(&[1, 2, 2], 1, 2), [2, 2, 2]);
        assert_eq!(
            replace_all(&["ooh", "la", "la"], "la", "baby"),
            ["ooh", "baby", "baby"]
        )
    }
}
