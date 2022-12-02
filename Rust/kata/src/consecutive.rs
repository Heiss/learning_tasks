struct Longest<'a> {
    index: usize,
    text: &'a str,
}

impl<'a> Longest<'a> {
    fn new(text: &'a str) -> Longest<'a> {
        Longest {
            index: 0,
            text,
        }
    }
}

impl<'a> std::iter::Iterator for Longest<'a> {
    type Item = (char, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.text.len() {
            return None;
        }

        let mut max = self.index + 1;
        let current = &self.text[self.index..self.index + 1];

        while max < self.text.len() && self.text[max..max + 1].eq(current) {
            max += 1;
        }
        let diff = max - self.index;
        self.index = max;

        Some((current.chars().next().expect("first char"), diff))
    }
}

pub fn longest_repetition(s: &str) -> Option<(char, usize)> {
    let mut longest = Longest::new(s);
    longest.fold(None, |acc, (c, len)| {
        match acc {
            Some((_, count)) =>
                (len > count).then(|| (c, len)).or_else(|| acc),
            _ => Some((c, len)),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::longest_repetition;

    #[test]
    fn longest_at_the_beginning() {
        assert_eq!(longest_repetition(&"aaaabbb"), Some(('a', 4)));
    }

    #[test]
    fn longest_at_the_end() {
        assert_eq!(longest_repetition(&"abbbbb"), Some(('b', 5)));
        assert_eq!(longest_repetition(&"bbbaaabaaaa"), Some(('a', 4)));
    }

    #[test]
    fn longest_in_the_middle() {
        assert_eq!(longest_repetition(&"cbdeuuu900"), Some(('u', 3)));
    }

    #[test]
    fn multiple_longest() {
        assert_eq!(longest_repetition(&"aabb"), Some(('a', 2)));
        assert_eq!(longest_repetition(&"ba"), Some(('b', 1)));
    }

    #[test]
    fn single_character_only() {
        assert_eq!(longest_repetition(&"aaaaaa"), Some(('a', 6)));
    }

    #[test]
    fn empty_string() {
        assert_eq!(longest_repetition(&""), None);
    }
}