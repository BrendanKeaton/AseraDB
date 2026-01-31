use crate::{
    enums::{Command, Filter, Operand, Syntax, TokenType, ValueTypes},
    structs::QueryObject,
};

use crate::handle_commands::handleSelect;

pub fn handle_sql_inputs(input: &str) -> bool {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut query: QueryObject = QueryObject::default();

    query.length = tokens.len() - 1;
    query.index = 0;

    while query.index <= query.length {
        let curr_token: TokenType = classify_token(tokens[query.index]);
        if let TokenType::CMD(cmd) = curr_token {
            if cmd == Command::SELECT {
                handleSelect(&tokens, &mut query);
            }
        }
        println!("{}", query);
        println!("{:?}", tokens[query.index]);
        query.index += 1;
    }

    return true;
}

pub fn classify_token(token: &str) -> TokenType {
    Command::from_str(token)
        .map(TokenType::CMD)
        .or_else(|| Filter::from_str(token).map(TokenType::FILTER))
        .or_else(|| Operand::from_str(token).map(TokenType::OP))
        .or_else(|| Syntax::from_str(token).map(TokenType::SYNTAX))
        .or_else(|| ValueTypes::from_str(token).map(TokenType::VALUE))
        .unwrap_or_else(|| return TokenType::CMD(Command::EXIT))
}
