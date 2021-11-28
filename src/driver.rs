use crate::{
    kaleidoscope_context::KaleidoscopeContext,
    lexer::{Lex, Lexer, Token},
    parser::{Parse, Parser},
};

use std::io::{Read, Write};

pub trait Drive {
    fn new(input: Box<dyn Read>, output: Box<dyn Write>) -> Self;
    fn run(&mut self) -> Result<(), std::io::Error>;
    fn handle_function_definition(&mut self) -> Result<(), std::io::Error>;
    fn handle_extern(&mut self) -> Result<(), std::io::Error>;
    fn handle_top_level_expression(&mut self) -> Result<(), std::io::Error>;
}

pub struct Driver {
    parser: Parser,
    lexer: Lexer<Box<dyn Read>>,
    context: KaleidoscopeContext,
    output: Box<dyn Write>,
}

impl Drive for Driver {
    fn new(input: Box<dyn Read>, output: Box<dyn Write>) -> Self {
        Driver {
            parser: Parser::new(),
            lexer: Lexer::new(input),
            context: KaleidoscopeContext::new(),
            output: output,
        }
    }
    fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            write!(self.output, "ready> ")?;
            self.output.flush()?;
            self.lexer.get_next_token();

            match self.lexer.current_token() {
                Some(Token::EOF) | None => return Ok(()),
                Some(Token::Misc(';')) => self.lexer.get_next_token(),
                Some(Token::Def) => self.handle_function_definition()?,
                Some(Token::Extern) => self.handle_extern()?,
                _ => self.handle_top_level_expression()?,
            }

            match self.lexer.current_token() {
                Some(Token::Misc(c)) => {
                    if *c != ';' {
                        writeln!(self.output, "Expected ';', but got {}", *c)?;
                    }
                }
                Some(tok) => writeln!(self.output, "Expected ';', but got {:#?}", tok)?,
                None => writeln!(self.output, "Expected ';', but got nothing...")?,
            }
        }
    }
    fn handle_function_definition(&mut self) -> Result<(), std::io::Error> {
        if self
            .parser
            .parse_function_definition(&mut self.lexer)
            .is_some()
        {
            writeln!(self.output, "Parsed a function definition")?;
            self.output.flush()?;
        } else {
            self.lexer.get_next_token();
        }
        Ok(())
    }

    fn handle_extern(&mut self) -> Result<(), std::io::Error> {
        if self.parser.parse_extern(&mut self.lexer).is_some() {
            writeln!(self.output, "Parsed an extern")?;
            self.output.flush()?;
        } else {
            self.lexer.get_next_token();
        }
        Ok(())
    }

    fn handle_top_level_expression(&mut self) -> Result<(), std::io::Error> {
        if self
            .parser
            .parse_top_level_expression(&mut self.lexer)
            .is_some()
        {
            writeln!(self.output, "Parsed a top level expression")?;
            self.output.flush()?;
        } else {
            self.lexer.get_next_token();
        }
        Ok(())
    }
}
