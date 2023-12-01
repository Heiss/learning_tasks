// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750

use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
enum InteractiveError {
    #[error("Invalid char: {0}")]
    TokenizerInvalidInput(char),
    #[error("No expression or function found")]
    ParserNoExpressionNorFunctionFound,
    #[error("Expected equal sign for assignment")]
    SyntaxErrorExpectedEqualSignForAssignment,
    #[error("Expected letter, underscore or digit. Found {}", .0)]
    SyntaxErrorExpectedLetterUnderscoreOrDigit(Token),
    #[error("Expected operator. Found {}", .0)]
    SyntaxErrorExpectedOperator(Token),
}

type Result<T> = std::result::Result<T, InteractiveError>;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    EqualSign,
    ParenOpen,
    ParenClose,
    Underscore,
    Dot,
    FnOperator,
    Letter(char),
    Digit(u8),
    FnKeyword,
}

#[derive(Clone)]
struct Tokenizer {
    tokens: Vec<Token>,
}

impl TryFrom<&str> for Tokenizer {
    type Error = InteractiveError;

    fn try_from(value: &str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            let val = match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '%' => Token::Modulo,
                '=' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        Token::FnOperator
                    } else {
                        Token::EqualSign
                    }
                }
                'f' => {
                    if let Some('n') = chars.peek() {
                        chars.next();
                        Token::FnKeyword
                    } else {
                        Token::Letter(c)
                    }
                }
                '(' => Token::ParenOpen,
                ')' => Token::ParenClose,
                '_' => Token::Underscore,
                '.' => Token::Dot,
                'a'..='z' | 'A'..='Z' => Token::Letter(c),
                '0'..='9' => {
                    let mut num = c.to_digit(10).unwrap() as u8;
                    Token::Digit(num)
                }
                ' ' => continue,
                v => return Err(InteractiveError::TokenizerInvalidInput(v)),
            };
            tokens.push(val);
        }
        Ok(Self { tokens })
    }
}

struct Function(Identifier, Vec<Identifier>, Box<Expression>);

impl TryFrom<Tokenizer> for Function {
    type Error = InteractiveError;

    fn try_from(value: Tokenizer) -> Result<Self> {
        todo!()
    }
}
type FnName = Identifier;

enum Expression {
    Factor(Box<Factor>),
    Operator(Box<Expression>, Operator, Box<Expression>),
}

impl TryFrom<&mut T> for Expression
where
    T: Iterator<Item = Token>,
{
    type Error = InteractiveError;

    fn try_from(value: &mut T) -> std::result::Result<Self, Self::Error> {
        let mut expr = Expression::Factor(Box::new(Factor::try_from(value)?));
        while let Some(op) = Operator::try_from(value).ok() {
            let rhs = Expression::Factor(Box::new(Factor::try_from(value)?));
            expr = Expression::Operator(Box::new(expr), op, Box::new(rhs));
        }
        Ok(expr)
    }
}

enum Factor {
    Number(Number),
    Identifier(Letter),
    Assignment(Assignment),
    Paren(Box<Expression>),
    FnCall(FnCall),
}

struct Assignment(Identifier, Box<Expression>);

impl<T> TryFrom<&mut T> for Assignment
where
    T: Iterator<Item = Token>,
{
    type Error = InteractiveError;

    fn try_from(value: &mut T) -> std::result::Result<Self, Self::Error> {
        let name = Identifier::try_from(value)?;
        if let Some(Token::EqualSign) = value.next() {
            let expr = Expression::try_from(value)?;
            Ok(Self(name, Box::new(expr)))
        } else {
            Err(InteractiveError::SyntaxErrorExpectedEqualSignForAssignment)
        }
    }
}

struct FnCall(Identifier, Vec<Box<Expression>>);

impl<T> TryFrom<&mut T> for FnCall
where
    T: Iterator<Item = Token>,
{
    type Error = InteractiveError;

    fn try_from(value: &mut T) -> std::result::Result<Self, Self::Error> {
        let name = FnName::try_from(value)?;
        let mut args = vec![];
    }
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

impl TryFrom<&Token> for Operator {
    type Error = InteractiveError;

    fn try_from(value: &Token) -> std::result::Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Plus),
            Token::Minus => Ok(Self::Minus),
            Token::Multiply => Ok(Self::Multiply),
            Token::Divide => Ok(Self::Divide),
            Token::Modulo => Ok(Self::Modulo),
            v => Err(InteractiveError::SyntaxErrorExpectedOperator(v.clone())),
        }
    }
}

enum Identifier {
    Letter(char),
    Underscore(Vec<Box<IdentifierChar>>),
}

impl<T> TryFrom<&mut T> for Identifier
where
    T: Iterator<Item = Token>,
{
    type Error = InteractiveError;

    fn try_from(value: &mut T) -> std::result::Result<Self, Self::Error> {
        match value.next() {
            Some(Token::Leter(c)) => Ok(Self::Letter(c)),
            Some(Token::Underscore) => {
                let mut arr = vec![];
                while let Some(v) = value.next() {
                    if let Ok(v) = IdentifierChar::try_from(&v) {
                        arr.push(Box::new(v));
                    } else {
                        break;
                    }
                }
                Ok(Self::Underscore(arr))
            }
        }
    }
}

enum IdentifierChar {
    Letter(char),
    Underscore,
    Digit(Digit),
}

impl TryFrom<&Token> for IdentifierChar {
    type Error = InteractiveError;

    fn try_from(value: &Token) -> std::result::Result<Self, Self::Error> {
        match value {
            Token::Letter(c) => Ok(Self::Letter(*c)),
            Token::Underscore => Ok(Self::Underscore),
            Token::Digit(d) => Ok(Self::Digit(Digit(*d))),
            _ => Err(InteractiveError::SyntaxErrorExpectedLetterUnderscoreOrDigit(value.clone())),
        }
    }
}

struct Number {
    decimal: Vec<Digit>,
    float: Vec<Digit>,
}
impl<T> TryFrom<&mut T> for Number
where
    T: Iterator<Item = Token>,
{
    type Error = InteractiveError;

    fn try_from(value: &mut T) -> std::result::Result<Self, Self::Error> {
        let mut decimal = vec![];
        let mut float = vec![];
        let mut is_float = false;
        while let Some(v) = value.next() {
            match v {
                Token::Digit(d) => {
                    if is_float {
                        float.push(Digit(d));
                    } else {
                        decimal.push(Digit(d));
                    }
                }
                Token::Dot => {
                    is_float = true;
                }
                _ => {
                    break;
                }
            }
        }
        Ok(Self { decimal, float })
    }
}
struct Letter(char);
struct Digit(u8);

enum AST {
    Expression(Expression),
    Function(Function),
}

struct Parser {
    ast: AST,
}

impl TryFrom<Tokenizer> for Parser {
    type Error = InteractiveError;

    fn try_from(value: Tokenizer) -> Result<Self> {
        let val = Expression::try_from(value.clone())
            .map(|e| AST::Expression(e))
            .or_else(|_| Function::try_from(value).map(|f| AST::Function(f)))
            .map_err(|_| InteractiveError::ParserNoExpressionNorFunctionFound)?;

        Ok(Self { ast: val })
    }
}

struct Interpreter {
    variables: HashMap<String, f32>,
    functions: HashMap<String, Vec<String>>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn input(&mut self, input: &str) -> std::result::Result<Option<f32>, String> {
        let tokenizer = Tokenizer::try_from(input)?;
        let mut tokens = tokenizer.tokens.into_iter();

        Ok(None)
    }
}

#[test]
fn basic_arithmetic() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
    assert_eq!(i.input("2 - 1"), Ok(Some(1.0)));
    assert_eq!(i.input("2 * 3"), Ok(Some(6.0)));
    assert_eq!(i.input("8 / 4"), Ok(Some(2.0)));
    assert_eq!(i.input("7 % 4"), Ok(Some(3.0)));
}

#[test]
fn variables() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
    assert_eq!(i.input("x"), Ok(Some(1.0)));
    assert_eq!(i.input("x + 3"), Ok(Some(4.0)));
    assert!(i.input("y").is_err());
}

#[test]
fn functions() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
    assert_eq!(i.input("avg 4 2"), Ok(Some(3.0)));
    assert!(i.input("avg 7").is_err());
    assert!(i.input("avg 7 2 4").is_err());
}

#[test]
fn conflicts() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
    assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
    assert!(i.input("fn x => 0").is_err());
    assert!(i.input("avg = 5").is_err());
}
