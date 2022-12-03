use std::vec::IntoIter;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ArrowHeadToken {
    LeftArrow,
    RightArrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TailToken {
    SingleTail,
    DoubleTail,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ArrowToken {
    Head(ArrowHeadToken),
    Tail(TailToken),
    Separator,
}

impl ArrowHeadToken {
    fn get_value(&self) -> i64 {
        match self {
            ArrowHeadToken::LeftArrow => -1,
            ArrowHeadToken::RightArrow => 1,
        }
    }
}

impl TailToken {
    fn get_value(&self) -> i64 {
        match self {
            TailToken::SingleTail => 1,
            TailToken::DoubleTail => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ArrowTokenError {
    InvalidToken(char),
}

#[derive(Debug, Clone, Copy)]
enum ArrowHeadTokenError {
    InvalidArrow,
}

#[derive(Debug, Clone, Copy)]
enum TailTokenError {
    InvalidTail,
}

impl From<&str> for ArrowToken {
    fn from(value: &str) -> Self {
        match value {
            "<" => ArrowToken::Head(ArrowHeadToken::LeftArrow),
            ">" => ArrowToken::Head(ArrowHeadToken::RightArrow),
            "-" => ArrowToken::Tail(TailToken::SingleTail),
            "=" => ArrowToken::Tail(TailToken::DoubleTail),
            _ => ArrowToken::Separator,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Arrow {
    start_token: Option<ArrowHeadToken>,
    end_token: Option<ArrowHeadToken>,
    tail_token: Option<TailToken>,
    count: usize,
}

impl Arrow {
    fn new() -> Self {
        Arrow {
            start_token: None,
            end_token: None,
            tail_token: None,
            count: 0,
        }
    }

    fn get_value(&self) -> i64 {
        if self.start_token.is_some() && self.end_token.is_some() {
            return 0;
        }

        if self.start_token.is_none() && self.end_token.is_none() {
            return 0;
        }

        let mut prefix = 1;
        if let Some(_start_token) = self.start_token {
            prefix *= -1;
        }

        if let Some(tail_token) = self.tail_token {
            return prefix * (tail_token.get_value() * (self.count as i64 + 1));
        } else {
            prefix
        }
    }
}

struct ArrowTokenizer {
    tokens: Vec<ArrowToken>,
}

impl ArrowTokenizer {
    fn new(text: &str) -> Self {
        let tokens = text
            .chars()
            .map(|c| ArrowToken::from(c.to_string().as_str()))
            .collect::<Vec<ArrowToken>>();

        ArrowTokenizer { tokens }
    }
}

impl IntoIterator for ArrowTokenizer {
    type Item = Arrow;
    type IntoIter = ArrowIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut arrows = Vec::new();
        let mut arrow = Arrow::new();

        for token in self.tokens {
            match token {
                ArrowToken::Head(ArrowHeadToken::LeftArrow) => {
                    if arrow.start_token.is_some() {
                        arrows.push(arrow);
                        arrow = Arrow::new();
                    }
                    arrow.start_token = Some(ArrowHeadToken::LeftArrow);
                    // Reset tails, because we have a new arrow
                    arrow.count = 0;
                    arrow.tail_token = None;
                }
                ArrowToken::Head(ArrowHeadToken::RightArrow) => {
                    if arrow.end_token.is_some() {
                        arrows.push(arrow);
                        arrow = Arrow::new();
                    }
                    arrow.end_token = Some(ArrowHeadToken::RightArrow);
                    // Always ends an arrow
                    arrows.push(arrow);
                    arrow = Arrow::new();
                }
                ArrowToken::Tail(tail) => {
                    if let Some(current_tail) = arrow.tail_token {
                        if current_tail != tail {
                            arrows.push(arrow);
                            arrow = Arrow::new();
                        }
                    }
                    arrow.tail_token = Some(tail);
                    arrow.count += 1;
                }
                ArrowToken::Separator => {
                    arrows.push(arrow);
                    arrow = Arrow::new();
                }
            }
        }
        arrows.push(arrow);
        ArrowIterator { arrows, index: 0 }
    }
}
struct ArrowIterator {
    index: usize,
    arrows: Vec<Arrow>,
}

impl Iterator for ArrowIterator {
    type Item = Arrow;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.arrows.len() {
            let arrow = self.arrows[self.index];
            self.index += 1;
            Some(arrow)
        } else {
            None
        }
    }
}

pub fn arrow_search(string: &str) -> i64 {
    ArrowTokenizer::new(string)
        .into_iter()
        .map(|a| a.get_value())
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::arrow_search;
    use std::fmt::Debug;

    fn pretty_assert_eq<T: PartialEq + Debug>(actual: T, expected: T) {
        assert_eq!(actual, expected, "{:?} should equal {:?}", actual, expected)
    }

    #[test]
    fn example_tests() {
        pretty_assert_eq(arrow_search(">."), 1);
        pretty_assert_eq(arrow_search("<--.."), -3);
        pretty_assert_eq(arrow_search("><=><--"), -2);
        pretty_assert_eq(arrow_search(">===>->"), 11);
        pretty_assert_eq(arrow_search("......"), 0);
        pretty_assert_eq(arrow_search("<-->"), 0);
        pretty_assert_eq(arrow_search("<."), -1);
        pretty_assert_eq(arrow_search("<<."), -2);
        pretty_assert_eq(arrow_search(">."), 1);
        pretty_assert_eq(arrow_search(">>."), 2);
        pretty_assert_eq(arrow_search("-<-"), -2);
        pretty_assert_eq(arrow_search("->-"), 2);
        pretty_assert_eq(arrow_search("<.-"), -1);
    }
}
