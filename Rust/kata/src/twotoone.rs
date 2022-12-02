/*
https://www.codewars.com/kata/5656b6906de340bd1b0000ac

DESCRIPTION:
Take 2 strings s1 and s2 including only letters from a to z. Return a new sorted string, the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.

Examples:
a = "xyaabbbccccdefww"
b = "xxxxyyyyabklmopq"
longest(a, b) -> "abcdefklmopqwxy"

a = "abcdefghijklmnopqrstuvwxyz"
longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
*/

fn longest(a1: &str, a2: &str) -> String {
    StringHelper::default().add(a1).add(a2).get_string()
}

#[derive(Default)]
struct StringHelper {
    current: Vec<char>,
}

impl StringHelper {
    fn add(&mut self, other: &str) -> &mut Self {
        self.current
            .append(&mut other.chars().collect::<Vec<char>>());
        self.current.sort_unstable();
        self.current.dedup();
        self
    }

    fn get_string(&self) -> String {
        self.current.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
