struct LongestConsec<'a> {
    strings: &'a [&'a str],
    k: usize,
}

enum LongestConsecError {
    InvalidK,
    EmptyStrings,
    LengthOfStringsSmallerAsK,
}

impl<'a> LongestConsec<'a> {
    fn create(strings: &'a [&'a str], k: usize) -> Result<LongestConsec<'a>, LongestConsecError> {
        if k == 0 {
            return Err(LongestConsecError::InvalidK);
        }
        if strings.is_empty() {
            return Err(LongestConsecError::EmptyStrings);
        }
        if strings.len() < k {
            return Err(LongestConsecError::LengthOfStringsSmallerAsK);
        }

        Ok(LongestConsec { strings, k })
    }

    fn longest_consec(&self) -> &[&str] {
        self.strings
            .windows(self.k)
            .map(|s| (s, s.iter().map(|s| s.len()).sum::<usize>()))
            .fold(None, |acc: Option<(&[&str], usize)>, s| match acc {
                Some((curr, count)) => (s.1 > count).then(|| s).or_else(|| (Some((curr, count)))),
                _ => Some(s),
            })
            .unwrap_or_default()
            .0
    }
}

// https://www.codewars.com/kata/56a5d994ac971f1ac500003e/python
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    LongestConsec::create(&strarr, k)
        .map(|lc| lc.longest_consec().join(""))
        .unwrap_or_default()
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    testing(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    testing(vec![], 3, "");
    testing(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
