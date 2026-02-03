use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    INSERT,
    SELECT,
    CREATE,
    EXIT,
}

impl Command {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "select" => Some(Command::SELECT),
            "insert" => Some(Command::INSERT),
            "create" => Some(Command::CREATE),
            "exit" => Some(Command::EXIT),
            _ => None,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::INSERT => write!(f, "INSERT"),
            Command::SELECT => write!(f, "SELECT"),
            Command::CREATE => write!(f, "CREATE"),
            Command::EXIT => write!(f, "EXIT"),
        }
    }
}
