use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    INSERT,
    SELECT,
    DELETE,
    CREATE,
    EXIT,
}

impl Command {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "select" => Some(Command::SELECT),
            "insert" => Some(Command::INSERT),
            "delete" => Some(Command::DELETE),
            "create" => Some(Command::CREATE),
            "exit" => Some(Command::EXIT),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    FROM,
    WHERE,
    INTO,
    //AND,
    //OR,
    //NOT,
    //SUM,
    //AVG,
}

impl Filter {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "from" => Some(Filter::FROM),
            "where" => Some(Filter::WHERE),
            "into" => Some(Filter::INTO),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueTypes {
    String(String),
    Number(i32),
}

// this from_str function will always return "Some"...
// This is so that down the line, we can update the parsing to be "better",
// IE using single or double quotes as a qualifier for strings,
// Updating to allow floats, etc, without having to change logic elsewhere (in utils.rs)

impl ValueTypes {
    pub fn from_str(s: &str) -> Option<Self> {
        if let Ok(num) = s.parse::<i32>() {
            Some(ValueTypes::Number(num))
        } else {
            Some(ValueTypes::String(s.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Operand {
    #[default]
    EQ,
    GE,
    GTE,
    LT,
    LTE,
    NQ,
}

impl Operand {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ">" => Some(Operand::GE),
            ">=" => Some(Operand::GTE),
            "=" | "==" => Some(Operand::EQ),
            "<" => Some(Operand::LT),
            "<=" => Some(Operand::LTE),
            "!=" | "<>" => Some(Operand::NQ),
            _ => None,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    CMD(Command),
    OP(Operand),
    FILTER(Filter),
    VALUE(ValueTypes),
    SYNTAX(Syntax),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::INSERT => write!(f, "INSERT"),
            Command::SELECT => write!(f, "SELECT"),
            Command::DELETE => write!(f, "DELETE"),
            Command::CREATE => write!(f, "CREATE"),
            Command::EXIT => write!(f, "EXIT"),
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::FROM => write!(f, "FROM"),
            Filter::WHERE => write!(f, "WHERE"),
            Filter::INTO => write!(f, "INTO"),
        }
    }
}

impl fmt::Display for ValueTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueTypes::String(s) => write!(f, "{}", s),
            ValueTypes::Number(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::EQ => write!(f, "="),
            Operand::GE => write!(f, ">"),
            Operand::GTE => write!(f, ">="),
            Operand::LT => write!(f, "<"),
            Operand::LTE => write!(f, "<="),
            Operand::NQ => write!(f, "!="),
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::CMD(cmd) => write!(f, "{}", cmd),
            TokenType::OP(op) => write!(f, "{}", op),
            TokenType::FILTER(filter) => write!(f, "{}", filter),
            TokenType::VALUE(val) => write!(f, "{}", val),
            TokenType::SYNTAX(syn) => write!(f, "{}", syn),
        }
    }
}
