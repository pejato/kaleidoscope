use crate::parser::Parser;
use std::io::{stdin, BufRead};
use tokenization::{Lexer, Token};

pub mod ast;
pub mod environment;
pub mod parser;
mod test_utilities;
pub mod tokenization;

fn handle_function_definition<T: BufRead>(parser: &mut Parser, lexer: &mut Lexer<T>) {
    if parser.parse_function_definition(lexer).is_some() {
        eprintln!("Parsed a function definition");
    } else {
        lexer.get_next_token();
    }
}

fn handle_extern<T: BufRead>(parser: &mut Parser, lexer: &mut Lexer<T>) {
    if parser.parse_extern(lexer).is_some() {
        eprintln!("Parsed an extern");
    } else {
        lexer.get_next_token();
    }
}

fn handle_top_level_expression<T: BufRead>(parser: &mut Parser, lexer: &mut Lexer<T>) {
    if parser.parse_top_level_expression(lexer).is_some() {
        eprintln!("Parsed a top level expression");
    } else {
        lexer.get_next_token();
    }
}

fn main() {
    let mut parser = Parser::new();
    let stdin = stdin();
    let instream_handle = stdin.lock();
    let mut lexer = Lexer::new(instream_handle);
    lexer.get_next_token();

    loop {
        eprint!("ready>");
        match lexer.current_token() {
            Some(Token::EOF) | None => return,
            Some(Token::Misc(';')) => lexer.get_next_token(),
            Some(Token::Def) => handle_function_definition(&mut parser, &mut lexer),
            Some(Token::Extern) => handle_extern(&mut parser, &mut lexer),
            _ => handle_top_level_expression(&mut parser, &mut lexer),
        }
    }
}
