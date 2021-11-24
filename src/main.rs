use {
    lexer::{Lex, Lexer, Token},
    parser::{Parse, Parser},
    std::io::{stdin, stdout, BufRead, Write},
};

pub mod ast;
pub mod environment;
pub mod lexer;
pub mod parser;
mod test_utilities;

trait Drive {
    fn new() -> Self;
    fn handle_function_definition<T: BufRead>(
        &self,
        parser: &mut Parser,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    );
    fn handle_extern<T: BufRead>(
        &self,
        parser: &mut Parser,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    );
    fn handle_top_level_expression<T: BufRead>(
        &self,
        parser: &mut Parser,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    );
}

struct Driver {}

impl Drive for Driver {
    fn new() -> Self {
        Driver {}
    }
    fn handle_function_definition<T: BufRead>(
        &self,
        parser: &mut Parser,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) {
        if parser.parse_function_definition(lexer).is_some() {
            write!(output, "Parsed a function definition").unwrap();
        } else {
            lexer.get_next_token();
        }
    }

    fn handle_extern<T: BufRead>(
        &self,
        parser: &mut Parser,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) {
        if parser.parse_extern(lexer).is_some() {
            write!(output, "Parsed an extern").unwrap();
        } else {
            lexer.get_next_token();
        }
    }

    fn handle_top_level_expression<T: BufRead>(
        &self,
        parser: &mut Parser,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) {
        if parser.parse_top_level_expression(lexer).is_some() {
            write!(output, "Parsed a top level expression").unwrap();
        } else {
            lexer.get_next_token();
        }
    }
}

fn main() {
    let driver = Driver::new();
    let mut parser = Parser::new();
    let stdin = stdin();
    let stdout = stdout();

    let mut lexer = Lexer::new(stdin.lock());
    lexer.get_next_token();

    loop {
        write!(&mut stdout.lock(), "ready>").unwrap();
        match lexer.current_token() {
            Some(Token::EOF) | None => return,
            Some(Token::Misc(';')) => lexer.get_next_token(),
            Some(Token::Def) => {
                driver.handle_function_definition(&mut parser, &mut lexer, &mut stdout.lock())
            }
            Some(Token::Extern) => {
                driver.handle_extern(&mut parser, &mut lexer, &mut stdout.lock())
            }
            _ => driver.handle_top_level_expression(&mut parser, &mut lexer, &mut stdout.lock()),
        }
    }
}
