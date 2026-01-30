use crate::{
    enums::{Command, Filter, Operand, TokenType},
    structs::QueryObject,
};

pub fn handle_sql_inputs(input: &str) -> bool {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut query: QueryObject = QueryObject::default();

    return true;
}

fn classify_token(token: &str) -> TokenType {
    Command::from_str(token)
        .map(TokenType::CMD)
        .or_else(|| Filter::from_str(token).map(TokenType::FILTER))
        .unwrap_or_else(|| return TokenType::OP(Operand::EQ))
}
