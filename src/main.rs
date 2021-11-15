use std::collections::HashMap;

use tokenization::{Token, TokenConsumer};

use crate::parser::Parser;

pub mod ast;
pub mod environment;
pub mod parser;
mod test_utilities;
pub mod tokenization;

fn main() {
    let mut parser = Parser::new();
    let mut lexer = TokenConsumer::new(Box::new(std::io::stdin()));
    lexer.consume_token();

    loop {
        eprint!("ready>");
        match lexer.current_token() {
            Some(Token::EOF) | None => return,
            Some(Token::Misc(';')) => {
                lexer.consume_token();
                ()
            }
            Some(Token::Def) => {
                parser.parse_function_definition(&mut lexer);
                ()
            }
            Some(Token::Extern) => {
                parser.parse_extern(&mut lexer);
                ()
            }
            _ => {
                parser.parse_top_level_expression(&mut lexer);
                ()
            }
        }
    }
}
