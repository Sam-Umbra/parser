use std::fmt::Display;

use crate::{
    lexer::{Lexer, Token},
    models::operators::Operator,
};

pub enum Expression {
    Atom(char),
    Operation(Operator, Vec<Expression>),
}

impl Expression {
    pub fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0)
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: u8) -> Expression {
    let mut left_side = match lexer.next() {
        Token::Atom(c) => Expression::Atom(c),
        at => panic!("Bad token: {:?}", at),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            at => panic!("Unexpected atom: {:?}", at),
        };
        lexer.next();
        let (l_bp, r_bp) = op.infix_binding_power();
        if l_bp < min_bp {
            break;
        }
        let right_side = parse_expression(lexer, r_bp);
        left_side = Expression::Operation(op, vec![left_side, right_side]);
    }
    left_side
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Atom(c) => write!(f, "{}", c),
            Expression::Operation(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}
