use std::fmt::Display;

use regex::regex;

use crate::{
    lexer::Atom::{Number, Variable},
    models::operators::{
        Infix::{ImplicitTimes, Minus, Plus, Power, Root, Slash, Times},
        Operator,
        Parenthesis::{Closing, Opening},
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

fn filter_input(input: &str) -> String {
    let mut clean_input = String::with_capacity(input.len() * 2);
    let mut prev: Option<char> = None;

    for c in input.chars() {
        if let Some(p) = prev {
            let left_ok = p.is_ascii_alphanumeric() || p == ')';
            let implicit = (left_ok && (c.is_ascii_alphabetic() || c == '('))
                || ((p.is_ascii_alphabetic() || p == ')') && c.is_ascii_digit());
            if implicit {
                clean_input.push('\'');
            }
        }
        clean_input.push(c);
        prev = Some(c);
    }
    clean_input
}
impl Lexer {
    pub fn new(input: &str) -> Lexer {
        if input.matches("=").count() > 1 {
            panic!("No more the 1 equals allowed")
        }
        if input.matches("'").count() > 0 {
            panic!("Unknown char")
        }

        let filtered_input = filter_input(input);

        let regex = regex!(
            r#"(?x)
        (?P<number>   \d+(?:\.\d+)? ) |
        (?P<variable> [a-zA-Z] )      |
        (?P<operator> [\+\-*/^√=()'] ) |
        (?P<unknown> \S)
    "#
        );

        let mut tokens: Vec<Token> = regex
            .captures_iter(&filtered_input)
            .map(|caps| {
                if let Some(unk) = caps.name("unknown") {
                    panic!("Unknown char: {:?}", unk.as_str())
                } else if let Some(n) = caps.name("number") {
                    Token::Atom(Number(n.as_str().parse::<f64>().unwrap()))
                } else if let Some(v) = caps.name("variable") {
                    Token::Atom(Variable(v.as_str().chars().next().unwrap()))
                } else {
                    let op = caps.name("operator").unwrap().as_str();
                    Token::Op(match op {
                        "+" => Operator::Infix(Plus),
                        "-" => Operator::Infix(Minus),
                        "*" => Operator::Infix(Times),
                        "'" => Operator::Infix(ImplicitTimes),
                        "/" => Operator::Infix(Slash),
                        "^" => Operator::Infix(Power),
                        "√" => Operator::Infix(Root),
                        "=" => Operator::Equals,
                        "(" => Operator::Parenthesis(Opening),
                        ")" => Operator::Parenthesis(Closing),
                        _ => unreachable!(),
                    })
                }
            })
            .collect();
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
