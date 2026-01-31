use crate::enums::TokenType;
use crate::utils::classify_token;
use crate::{enums::Command, structs::QueryObject};

pub fn handleSelect(tokens: &Vec<&str>, query: &mut QueryObject) {
    query.command = Some(Command::SELECT);
    query.index += 1;

    while let TokenType::VALUE(val) = classify_token(tokens[query.index]) {
        // val contains the ValueTypes enum value here
        query.fields.push(val);
        query.index += 1;
    }
}
