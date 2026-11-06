use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Infix(Infix),
    Parenthesis(Parenthesis),
    Equals, /*
            Power,
            Factorial,
            Modulus, // Rest of div */
}

impl Operator {
    pub fn infix_binding_power(&self) -> (u8, u8) {
        match self {
            Self::Equals => (1, 0),
            Self::Infix(inf) => {
                let bp = inf.binding_power();
                bp
            }
            Self::Parenthesis(_) => (0, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parenthesis {
    Opening,
    Closing,
}

impl Display for Parenthesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            Self::Opening => "(",
            Self::Closing => ")",
        };
        write!(f, "{}", p)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Slash,
    Times,
    Power,
    Root,
}

impl Infix {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Plus | Self::Minus => (2, 3),
            Self::Slash | Self::Times => (4, 5),
            Self::Power | Self::Root => (7, 6),
        }
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Slash => "/",
            Self::Times => "*",
            Self::Power => "^",
            Self::Root => "√",
        };
        write!(f, "{}", symbol)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Infix(inf) => write!(f, "{}", inf),
            Operator::Parenthesis(p) => write!(f, "{}", p),
            Operator::Equals => write!(f, "="),
        }
    }
}
