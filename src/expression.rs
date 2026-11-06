use std::{collections::HashMap, fmt::Display};

use crate::{
    lexer::{
        Atom::{self, Number, Variable},
        Lexer, Token,
    },
    models::operators::{
        Infix::{Minus, Plus, Power, Root, Slash, Times},
        Operator,
        Parenthesis::{Closing, Opening},
    },
};

pub enum Expression {
    Atom(Atom),
    Operation(Operator, Vec<Expression>),
}

impl Expression {
    pub fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0)
    }

    pub fn is_assign(&self) -> Option<(char, &Expression)> {
        match self {
            Expression::Atom(_) => return None,
            Expression::Operation(op, operands) => {
                if op.eq(&Operator::Equals) {
                    let var = match operands.first().unwrap() {
                        Expression::Atom(Variable(v)) => *v,
                        _ => unreachable!(),
                    };
                    return Some((var, operands.last().unwrap()));
                }
                return None;
            }
        }
    }

    pub fn eval(&self, variables: &HashMap<char, f64>) -> f64 {
        match self {
            Expression::Atom(at) => match at {
                Number(num) => *num,
                Variable(var) => *variables
                    .get(var)
                    .expect(&format!("Undefined variable {}", var)),
            },
            Expression::Operation(operator, operands) => {
                let left_side = operands.first().unwrap().eval(variables);
                let right_side = operands.last().unwrap().eval(variables);
                match operator {
                    Operator::Infix(inf) => match inf {
                        Plus => return left_side + right_side,
                        Minus => return left_side - right_side,
                        Slash => return left_side / right_side,
                        Times => return left_side * right_side,
                        Power => return left_side.powf(right_side),
                        Root => return right_side.powf(1 as f64 / left_side),
                    },
                    Operator::Parenthesis(_) | Operator::Equals => {
                        panic!("Operator cannot be evaluated as an expression")
                    }
                }
            }
        }
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: u8) -> Expression {
    let mut left_side = match lexer.next() {
        Token::Atom(c) => Expression::Atom(c),
        Token::Op(Operator::Parenthesis(Opening)) => {
            let left_side = parse_expression(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(Operator::Parenthesis(Closing)));
            left_side
        }
        at => panic!("Bad token: {:?}", at),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(Operator::Parenthesis(Closing)) => break,
            Token::Op(op) => op,
            at => panic!("Unexpected atom: {:?}", at),
        };
        let (l_bp, r_bp) = op.infix_binding_power();
        if l_bp < min_bp {
            break;
        }
        lexer.next();
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
