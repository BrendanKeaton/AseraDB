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

pub enum ValueTypes {
    String(String),
    Number(i32),
}

pub enum Operand {
    GE,
    GTE,
    EQ,
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

pub enum TokenType {
    CMD(Command),
    OP(Operand),
    FILTER(Filter),
    VALUE(ValueTypes),
}
