use crate::{
    lexer::{Lex, Lexer, Token},
    parser::{Parse, Parser},
};

use std::io::{stdin, stdout, Read, Write};

pub trait Drive {
    fn new() -> Self;
    fn run(&mut self) -> Result<(), std::io::Error>;
    fn handle_function_definition<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error>;
    fn handle_extern<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error>;
    fn handle_top_level_expression<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error>;
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
    fn run(&mut self) -> Result<(), std::io::Error> {
        let stdin = stdin();
        let mut stdout = stdout();
        let mut lexer = Lexer::new(stdin.lock());

        loop {
            write!(&mut stdout, "ready> ")?;
            stdout.flush()?;
            lexer.get_next_token();

            match lexer.current_token() {
                Some(Token::EOF) | None => return Ok(()),
                Some(Token::Misc(';')) => lexer.get_next_token(),
                Some(Token::Def) => self.handle_function_definition(&mut lexer, &mut stdout)?,
                Some(Token::Extern) => self.handle_extern(&mut lexer, &mut stdout)?,
                _ => self.handle_top_level_expression(&mut lexer, &mut stdout)?,
            }
        }
    }
    fn handle_function_definition<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        if self.parser.parse_function_definition(lexer).is_some() {
            writeln!(output, "Parsed a function definition")?;
            output.flush()?;
        } else {
            lexer.get_next_token();
        }
        Ok(())
    }

    fn handle_extern<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        if self.parser.parse_extern(lexer).is_some() {
            writeln!(output, "Parsed an extern")?;
            output.flush()?;
        } else {
            lexer.get_next_token();
        }
        Ok(())
    }

    fn handle_top_level_expression<T: Read>(
        &mut self,
        lexer: &mut Lexer<T>,
        output: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        if self.parser.parse_top_level_expression(lexer).is_some() {
            writeln!(output, "Parsed a top level expression")?;
            output.flush()?;
        } else {
            lexer.get_next_token();
        }
        Ok(())
    }
}
