// https://www.codewars.com/kata/586dd26a69b6fd46dd0000c0

struct MemoryCell(u8);

#[derive(PartialEq)]
enum Token {
    Increment,
    Print,
    Invalid,
}

struct Parser {
    tokens: Vec<Token>,
}

struct Tokenizer {
    tokens: Vec<Token>,
}

impl MemoryCell {
    fn new() -> Self {
        Self(0)
    }

    fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

impl ToString for MemoryCell {
    fn to_string(&self) -> String {
        (self.0 as char).to_string()
    }
}

impl From<&str> for Tokenizer {
    fn from(code: &str) -> Self {
        let tokens = code
            .chars()
            .map(|c| match c {
                '+' => Token::Increment,
                '.' => Token::Print,
                _ => Token::Invalid,
            })
            .collect();
        Self { tokens }
    }
}

impl From<Tokenizer> for Parser {
    fn from(tokenizer: Tokenizer) -> Self {
        let tokens = tokenizer
            .tokens
            .into_iter()
            .filter(|token| *token != Token::Invalid)
            .collect();
        Self { tokens }
    }
}

impl Parser {
    fn execute(self) -> String {
        let mut memory_cell = MemoryCell::new();
        let mut output: String = String::new();
        self.tokens.into_iter().for_each(|token| match token {
            Token::Increment => memory_cell.increment(),
            Token::Print => output.push_str(&memory_cell.to_string()),
            _ => (),
        });
        output
    }
}

fn my_first_interpreter(code: &str) -> String {
    let tokenizer = Tokenizer::from(code);
    let parser = Parser::from(tokenizer);
    parser.execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_cases() {
        // Outputs the uppercase English alphabet
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+."), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        // Hello World Program - outputs the string "Hello, World!"
        assert_eq!(my_first_interpreter("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++."), "Hello, World!");
    }
}
