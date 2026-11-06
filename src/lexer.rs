use std::fmt::Display;

use crate::{
    lexer::Atom::{Number, Variable}, models::operators::{
        Infix::{Minus, Plus, Power, Root, Slash, Times}, Operator, Parenthesis::{Closing, Opening},
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Atom(Atom),
    Op(Operator),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Atom {
    Number(f64),
    Variable(char),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            Self::Number(n) => n.to_string(),
            Self::Variable(v) => v.to_string(),
        };
        write!(f, "{}", a)
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens: Vec<Token> = input
            .chars()
            .filter(|s| !s.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' => Token::Atom(Number(c.to_digit(10).unwrap() as f64)),
                'a'..='z' | 'A'..='Z' => Token::Atom(Variable(c)),
                '+' => Token::Op(Operator::Infix(Plus)),
                '-' => Token::Op(Operator::Infix(Minus)),
                '/' => Token::Op(Operator::Infix(Slash)),
                '*' => Token::Op(Operator::Infix(Times)),
                '(' => Token::Op(Operator::Parenthesis(Opening)),
                ')' => Token::Op(Operator::Parenthesis(Closing)),
                '^' => Token::Op(Operator::Infix(Power)),
                '√' => Token::Op(Operator::Infix(Root)),
                '=' => Token::Op(Operator::Equals),
                _ => panic!("Unknown char: {:?}", c),
            })
            .collect::<Vec<Token>>();
        tokens.reverse();
        Lexer { tokens }
    }

    // Consumes the next token
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    // Sees the next token without consuming it
    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}
