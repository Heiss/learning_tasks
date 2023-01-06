struct Disemvowel<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> From<&'a str> for Disemvowel<'a> {
    fn from(input: &'a str) -> Self {
        Self { input, index: 0 }
    }
}

fn is_ascii_vowel(c: &char) -> bool {
    match c.to_lowercase().next().expect("No char given") {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

impl<'a> Iterator for Disemvowel<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.input.len() {
            let c = self.input.chars().nth(self.index)?;
            self.index += 1;
            if !is_ascii_vowel(&c) {
                return Some(c);
            }
        }
        None
    }
}

fn disemvowel(s: &str) -> String {
    Disemvowel::from(s).collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
