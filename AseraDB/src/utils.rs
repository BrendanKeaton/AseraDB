use crate::{
    enums::{Command, Filter, Operand, Syntax, TokenType, ValueTypes},
    handle_commands::handle_create,
    structs::QueryObject,
};

use crate::handle_commands::handle_select;

pub fn handle_sql_inputs(input: &str, query: &mut QueryObject) -> bool {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    query.length = tokens.len();
    query.index = 0;

    while query.index < query.length {
        let curr_token: TokenType = classify_token(tokens[query.index]);
        match curr_token {
            TokenType::CMD(command) => {
                let cmd_result = match_command(&command, &tokens, query);
                if let Err(e) = cmd_result {
                    println!("Malformed Request. {}", e);
                }
            }
            TokenType::OP(operand) => todo!(),
            TokenType::FILTER(filter) => todo!(),
            TokenType::VALUE(value_types) => todo!(),
            TokenType::SYNTAX(syntax) => todo!(),
        }
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

fn match_command(
    command: &Command,
    tokens: &[&str],
    query: &mut QueryObject,
) -> Result<(), String> {
    match command {
        Command::SELECT => handle_select(tokens, query),
        Command::INSERT => todo!(),
        Command::DELETE => todo!(),
        Command::CREATE => handle_create(tokens, query),
        Command::EXIT => todo!(),
    }
}

fn match_filter(filter: &Filter, tokens: &[&str], query: &mut QueryObject) -> Result<(), String> {
    match filter {
        Filter::FROM => todo!(),
        Filter::INTO => todo!(),
        Filter::WHERE => todo!(),
    }
}
