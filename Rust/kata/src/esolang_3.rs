// https://www.codewars.com/kata/5868a68ba44cfc763e00008d

use itertools::Itertools;

#[derive(Clone, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn not(&self) -> Self {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Tokens {
    Up,
    Right,
    Down,
    Left,
    Flip,
    Invalid,
    JumpForward,
    JumpBackward,
}

struct Tokenizer {
    tokens: Vec<Tokens>,
}

impl From<&str> for Tokenizer {
    fn from(code: &str) -> Self {
        let tokens = code
            .chars()
            .map(|c| match c {
                'n' => Tokens::Up,
                'e' => Tokens::Right,
                's' => Tokens::Down,
                'w' => Tokens::Left,
                '*' => Tokens::Flip,
                '[' => Tokens::JumpForward,
                ']' => Tokens::JumpBackward,
                _ => Tokens::Invalid,
            })
            .collect();
        Self { tokens }
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Bit>,
}

impl Grid {
    fn get_value(&self, position: &Position) -> &Bit {
        let (x, y) = position;
        &self.cells[y * self.width + x]
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut output = Vec::new();
        for cell in self.cells.iter() {
            match cell {
                Bit::Zero => output.push("0".to_string()),
                Bit::One => output.push("1".to_string()),
            }
        }
        output
            .chunks(self.width)
            .map(|chunk| chunk.iter().join(""))
            .join("\r\n")
    }
}

struct Parser {
    tokens: Vec<Tokens>,
    index: usize,
    grid: Grid,
    position: Position,
}

type Iterations = usize;
type Position = (usize, usize);
type Size = (usize, usize);
type Input = (Tokenizer, Size);

impl From<Input> for Parser {
    fn from((tokenizer, (width, height)): Input) -> Self {
        let tokens = tokenizer
            .tokens
            .into_iter()
            .filter(|token| *token != Tokens::Invalid)
            .collect();
        let cells = vec![Bit::Zero; width * height];
        let grid = Grid {
            width,
            height,
            cells,
        };
        Self {
            tokens,
            index: 0,
            grid,
            position: (0, 0),
        }
    }
}

impl Parser {
    fn execute(&mut self, iterations: Iterations) -> String {
        for _ in 0..iterations {
            self.step();
        }
        self.grid.to_string()
    }

    fn step(&mut self) {
        match self.tokens.get(self.index) {
            Some(Tokens::Up) => self.move_position(&Tokens::Up),
            Some(Tokens::Right) => self.move_position(&Tokens::Right),
            Some(Tokens::Down) => self.move_position(&Tokens::Down),
            Some(Tokens::Left) => self.move_position(&Tokens::Left),
            Some(Tokens::Flip) => self.flip(),
            Some(Tokens::JumpForward) => self.move_cursor_forward(),
            Some(Tokens::JumpBackward) => self.move_cursor_backward(),
            _ => (),
        }
        self.index += 1;
    }

    fn move_position(&mut self, direction: &Tokens) {
        match direction {
            Tokens::Up => {
                if self.position.1 > 0 {
                    self.position.1 -= 1;
                } else {
                    self.position.1 = self.grid.height - 1;
                }
            }
            Tokens::Right => {
                if self.position.0 < self.grid.width - 1 {
                    self.position.0 += 1;
                } else {
                    self.position.0 = 0;
                }
            }
            Tokens::Down => {
                if self.position.1 < self.grid.height - 1 {
                    self.position.1 += 1;
                } else {
                    self.position.1 = 0;
                }
            }
            Tokens::Left => {
                if self.position.0 > 0 {
                    self.position.0 -= 1;
                } else {
                    self.position.0 = self.grid.width - 1;
                }
            }
            _ => (),
        }
    }

    fn flip(&mut self) {
        let index = self.position.0 + self.position.1 * self.grid.width;
        if let Some(v) = self.grid.cells.get_mut(index) {
            *v = v.not();
        };
    }

    fn move_cursor_forward(&mut self) {
        if self.grid.get_value(&self.position) == &Bit::One {
            return;
        }

        let mut braces_stack = Vec::new();
        let mut index = self.index;
        while index < self.tokens.len() {
            match self.tokens.get(index) {
                Some(Tokens::JumpForward) => braces_stack.push(index),
                Some(Tokens::JumpBackward) => {
                    braces_stack.pop();
                    if braces_stack.is_empty() {
                        break;
                    }
                }
                _ => (),
            }
            index += 1;
        }
        self.index = index;
    }

    fn move_cursor_backward(&mut self) {
        if self.grid.get_value(&self.position) == &Bit::Zero {
            return;
        }

        let mut braces_stack = Vec::new();
        let mut index = self.index;
        while index > 0 {
            match self.tokens.get(index) {
                Some(Tokens::JumpBackward) => braces_stack.push(index),
                Some(Tokens::JumpForward) => {
                    braces_stack.pop();
                    if braces_stack.is_empty() {
                        break;
                    }
                }
                _ => (),
            }
            index -= 1;
        }
        self.index = index;
    }
}

fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let tokenizer = Tokenizer::from(code);
    let mut parser = Parser::from((tokenizer, (width, height)));
    parser.execute(iterations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9)), display_expected("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)), display_expected("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9)), display_expected("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
        assert_eq!(display_actual(&interpreter("*", 10, 6, 9)), display_expected("100000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "only first bit");
        assert_eq!(display_actual(&interpreter("se*", 10, 6, 9)), display_expected("000000\r\n010000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "only second line, second bit");
        assert_eq!(display_actual(&interpreter("[]*", 10, 6, 9)), display_expected("100000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Skips everything, only first bit");
        assert_eq!(display_actual(&interpreter("*[es*]", 5, 6, 9)), display_expected("100000\r\n010000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Skips everything");
    }

    /// Prints representation of datagrid - 0's are black and 1's are white.
    /// Note: it only works properly if your interpreter returns a representation
    /// of the datagrid in the correct format.
    fn pretty_print(datagrid: &str) -> &str {
        let rows = datagrid.split("\r\n");
        let mut output = String::new();
        output += "<pre>";
        for row in rows {
            for cell in row.chars() {
                output += "<span style=\"color:";
                output += if cell == '0' { "black" } else { "white" };
                output += ";background-color:";
                output += if cell == '0' { "black" } else { "white" };
                output += "\">xx</span>";
            }
            output += "<br />";
        }
        output += "</pre>";
        println!("{}", output);
        datagrid
    }

    /// Displays the grid the interpreter returns
    fn display_actual(actual: &str) -> &str {
        println!("You returned:");
        pretty_print(actual)
    }

    /// Displays the expected final state of datagrid
    fn display_expected(expected: &str) -> &str {
        println!("Expected final state of data grid:");
        pretty_print(expected)
    }
}
