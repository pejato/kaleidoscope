use crate::{
    lexer::{Lex, Lexer, Token},
    parser::{Parse, Parser},
};

use std::io::{stdin, stdout, Read, Write};

pub trait Drive {
    fn new() -> Self;
    fn run(&mut self);
    fn handle_function_definition<T: Read>(&mut self, lexer: &mut Lexer<T>, output: &mut dyn Write);
    fn handle_extern<T: Read>(&mut self, lexer: &mut Lexer<T>, output: &mut dyn Write);
    fn handle_top_level_expression<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    );
}

pub struct Driver {
    parser: Parser,
}

impl Drive for Driver {
    fn new() -> Self {
        Driver {
            parser: Parser::new(),
        }
    }
    fn run(&mut self) {
        let stdin = stdin();
        let stdout = stdout();

        let mut lexer = Lexer::new(stdin.lock());
        lexer.get_next_token();

        loop {
            write!(&mut stdout.lock(), "ready>").unwrap();
            match lexer.current_token() {
                Some(Token::EOF) | None => return,
                Some(Token::Misc(';')) => lexer.get_next_token(),
                Some(Token::Def) => self.handle_function_definition(&mut lexer, &mut stdout.lock()),
                Some(Token::Extern) => self.handle_extern(&mut lexer, &mut stdout.lock()),
                _ => self.handle_top_level_expression(&mut lexer, &mut stdout.lock()),
            }
        }
    }
    fn handle_function_definition<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) {
        if self.parser.parse_function_definition(lexer).is_some() {
            write!(output, "Parsed a function definition").unwrap();
        } else {
            lexer.get_next_token();
        }
    }

    fn handle_extern<T: Read>(&mut self, lexer: &mut Lexer<T>, output: &mut dyn Write) {
        if self.parser.parse_extern(lexer).is_some() {
            write!(output, "Parsed an extern").unwrap();
        } else {
            lexer.get_next_token();
        }
    }

    fn handle_top_level_expression<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) {
        if self.parser.parse_top_level_expression(lexer).is_some() {
            write!(output, "Parsed a top level expression").unwrap();
        } else {
            lexer.get_next_token();
        }
    }
}
