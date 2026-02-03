use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    FROM,
    WHERE,
}

impl Filter {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "from" => Some(Filter::FROM),
            "where" => Some(Filter::WHERE),
            _ => None,
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::FROM => write!(f, "FROM"),
            Filter::WHERE => write!(f, "WHERE"),
        }
    }
}
