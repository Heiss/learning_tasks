// https://www.codewars.com/kata/5861487fdb20cff3ab000030

use itertools::Itertools;

#[derive(PartialEq, Clone)]
enum Tokens {
    Print,
    Read,
    MoveLeft,
    MoveRight,
    JumpForward,
    JumpBackward,
    Invalid,
    Flip,
}

struct Tokenizer {
    tokens: Vec<Tokens>,
}

impl From<&str> for Tokenizer {
    fn from(code: &str) -> Self {
        let tokens = code
            .chars()
            .map(|c| match c {
                ';' => Tokens::Print,
                ',' => Tokens::Read,
                '<' => Tokens::MoveLeft,
                '>' => Tokens::MoveRight,
                '[' => Tokens::JumpForward,
                ']' => Tokens::JumpBackward,
                '+' => Tokens::Flip,
                _ => Tokens::Invalid,
            })
            .collect();
        Self { tokens }
    }
}

#[derive(Clone, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn not(&mut self) {
        *self = match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

struct Parser {
    tokens: Vec<Tokens>,
    input: Vec<Bit>,
    tape: Vec<Bit>,
    cursor: usize,
}

impl From<Tokenizer> for Parser {
    fn from(tokenizer: Tokenizer) -> Self {
        let tokens = tokenizer
            .tokens
            .into_iter()
            .filter(|token| *token != Tokens::Invalid)
            .collect();
        Self {
            tokens,
            tape: vec![Bit::Zero],
            cursor: 0,
            input: vec![],
        }
    }
}

impl ToString for Parser {
    fn to_string(&self) -> String {
        self.tape
            .iter()
            .map(|bit| match bit {
                Bit::Zero => '0',
                Bit::One => '1',
            })
            .chunks(8)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .map(|chunk| u8::from_str_radix(&chunk, 2).unwrap() as char)
            .collect()
    }
}

impl Parser {
    fn execute(&mut self, input: Vec<u8>) -> Vec<u8> {
        self.input = input
            .into_iter()
            .map(|v| format!("{:#08b}", v).replace("0b", ""))
            .map(|v| {
                v.chars()
                    .map(|c| match c {
                        '0' => Bit::Zero,
                        _ => Bit::One,
                    })
                    .collect::<Vec<Bit>>()
            })
            .flatten()
            .collect();

        let mut output = vec![];
        let mut tokens = std::mem::take(&mut self.tokens);

        while let Some(token) = tokens.pop() {
            match token {
                Tokens::Print => output.push(self.print()),
                Tokens::Read => self.read(),
                Tokens::MoveLeft => self.move_left(),
                Tokens::MoveRight => self.move_right(),
                Tokens::JumpForward => self.jump_forward(),
                Tokens::JumpBackward => self.jump_backward(),
                Tokens::Flip => self.flip(),
                _ => (),
            }
        }

        output
            .chunks(8)
            .into_iter()
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|v| match v {
                        Bit::Zero => '0',
                        Bit::One => '1',
                    })
                    .collect::<String>()
            })
            .map(|chunk| u8::from_str_radix(&chunk, 2).unwrap())
            .collect()
    }

    fn print(&self) -> Bit {
        self.tape[self.cursor].clone()
    }

    fn read(&mut self) {
        if self.input.is_empty() {
            self.tape[self.cursor] = Bit::Zero;
        } else {
            self.tape[self.cursor] = self.input.pop().unwrap();
        }
    }

    fn flip(&mut self) {
        self.tape[self.cursor].not();
    }

    fn move_left(&mut self) {
        if self.cursor == 0 {
            self.tape.insert(0, Bit::Zero);
        } else {
            self.cursor -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor == self.tape.len() - 1 {
            self.tape.push(Bit::Zero);
        }
        self.cursor += 1;
    }

    fn jump_forward(&mut self) {
        if self.tape[self.cursor] == Bit::Zero {
            return;
        }

        let mut depth = 0;
        for (i, token) in self.tokens.iter().enumerate() {
            match token {
                Tokens::JumpForward => depth += 1,
                Tokens::JumpBackward => depth -= 1,
                _ => (),
            }
            if depth == 0 {
                self.cursor = i;
                break;
            }
        }
    }

    fn jump_backward(&mut self) {
        if self.tape[self.cursor] == Bit::One {
            return;
        }

        let mut depth = 0;
        for (i, token) in self.tokens.iter().enumerate().rev() {
            match token {
                Tokens::JumpForward => depth -= 1,
                Tokens::JumpBackward => depth += 1,
                _ => (),
            }
            if depth == 0 {
                self.cursor = i;
                break;
            }
        }
    }
}

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let tokenizer = Tokenizer::from(code);
    let mut parser = Parser::from(tokenizer);
    let output = parser.execute(input);
    println!("{:?}", unsafe { std::str::from_utf8_unchecked(&output) });
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_cases() {
        assert_eq!(
            boolfuck(";", Vec::new()),
            b"",
            "Empty program should return empty string"
        );
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}
