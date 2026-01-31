use crate::enums::TokenType;
use crate::utils::classify_token;
use crate::{enums::Command, structs::QueryObject};

pub fn handle_select(tokens: &Vec<&str>, query: &mut QueryObject) {
    query.command = Some(Command::SELECT);
    query.index += 1;
    while let TokenType::VALUE(val) = classify_token(tokens[query.index]) {
        query.fields.push(val);
        query.index += 1;

        if query.index == query.length {
            println!(
                "Malformed Request. Please Complete Command. Commands cannot end with SELECT query."
            );
            return;
        }
    }
}

pub fn handle_create(tokens: &Vec<&str>, query: &mut QueryObject) {
    query.command = Some(Command::CREATE);
    query.index += 1;
    while let TokenType::VALUE(val) = classify_token(tokens[query.index]) {
        query.fields.push(val);
        query.index += 1;

        if query.index == query.length {
            println!(
                "Malformed Request. Please Complete Command. Commands cannot end with SELECT query."
            );
            return;
        }
    }
}
