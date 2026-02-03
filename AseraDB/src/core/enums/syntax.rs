use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    COMMA,
    STAR,
}

impl Syntax {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "," => Some(Syntax::COMMA),
            "*" => Some(Syntax::STAR),
            _ => None,
        }
    }
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Syntax::COMMA => write!(f, ","),
            Syntax::STAR => write!(f, "*"),
        }
    }
}
