use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Infix(Infix),
    /* Plus,
    Minus,
    Slash, // Div
    Times,
    Power,
    Factorial,
    Modulus, // Rest of div */
}

impl Operator {
    pub fn infix_binding_power(&self) -> (u8, u8) {
        match self {
            Self::Infix(inf) => {
                let bp = inf.binding_power();
                bp
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Infix {
    Plus,
    Minus,
    Slash,
    Times,
}

impl Infix {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Plus | Self::Minus => (1, 2),
            Self::Slash | Self::Times => (3, 4),
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
        };
        write!(f, "{}", symbol)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Infix(inf) => write!(f, "{}", inf),
        }
    }
}
