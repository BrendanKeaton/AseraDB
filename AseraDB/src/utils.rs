use crate::{
    enums::{Command, Filter, Operand, Syntax, TokenType, ValueTypes},
    structs::QueryObject,
};

use crate::handle_commands::handle_select;

pub fn handle_sql_inputs(input: &str) -> bool {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut query: QueryObject = QueryObject::default();

    query.length = tokens.len();
    query.index = 0;

    while query.index < query.length {
        let curr_token: TokenType = classify_token(tokens[query.index]);
        match curr_token {
            TokenType::CMD(command) => {
                match_command(&command, &tokens, &mut query);
            }
            TokenType::OP(operand) => todo!(),
            TokenType::FILTER(filter) => todo!(),
            TokenType::VALUE(value_types) => todo!(),
            TokenType::SYNTAX(syntax) => todo!(),
        }
        println!("{}", query)
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

fn match_command(command: &Command, tokens: &Vec<&str>, query: &mut QueryObject) {
    match command {
        Command::SELECT => handle_select(tokens, query),
        Command::INSERT => todo!(),
        Command::DELETE => todo!(),
        Command::CREATE => todo!(),
        Command::EXIT => todo!(),
    }
}
