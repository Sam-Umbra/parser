use crate::models::operators::{Infix, Operator};

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Atom(char),
    Op(Operator),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|s| !s.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                '+' => Token::Op(Operator::Infix(Infix::Plus)),
                '-' => Token::Op(Operator::Infix(Infix::Minus)),
                '/' => Token::Op(Operator::Infix(Infix::Slash)),
                '*' => Token::Op(Operator::Infix(Infix::Times)),
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
