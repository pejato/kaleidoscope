use std::{
    io::{ErrorKind, Read},
    string::String,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // TODO: Do we need this token?
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    If,
    Then,
    Else,
    Misc(char),
}

pub trait Lex {
    type Reader: Read;

    fn new(reader: Self::Reader) -> Self;
    fn get_next_token(&mut self) -> &Option<Token>;
    fn current_token(&self) -> &Option<Token>;
}

pub struct Lexer<T>
where
    T: Read,
{
    reader: T,
    buffer: Option<Token>,
    char_buffer: Option<char>,
    byte_buffer: [u8; 1],
}

// Public Interface

impl<T> Lex for Lexer<T>
where
    T: Read,
{
    type Reader = T;

    fn new(reader: T) -> Self {
        Lexer {
            reader,
            buffer: None,
            char_buffer: None,
            byte_buffer: [0],
        }
    }

    fn get_next_token(&mut self) -> &Option<Token> {
        self.buffer = self.get_token();
        &self.buffer
    }

    fn current_token(&self) -> &Option<Token> {
        &self.buffer
    }
}

// Private methods

#[macro_export]
macro_rules! read_exact {
    ($reader: expr, $buf:expr) => {
        match $reader.read_exact(&mut $buf) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => Err(e),
            Err(_) => {
                eprintln!("Failed to read character while Lexing, exiting...\n");
                std::process::exit(1);
            }
        }
    };
}

impl<T> Lexer<T>
where
    T: Read,
{
    fn is_newline(c: Option<char>) -> bool {
        if c.is_none() {
            return false;
        }
        let c = c.unwrap();

        c == '\n' || c == '\r'
    }

    // Methods

    fn get_token(&mut self) -> Option<Token> {
        let ch: char;

        // Check if there's a non-whitespace char already in the buffer
        if self.char_buffer.map_or(true, |c| c.is_ascii_whitespace()) {
            match self.try_get_char(true) {
                Some(c) => ch = c,
                None => return Token::EOF.into(),
            }
        } else {
            ch = self.char_buffer.unwrap();
        }

        // Def, Extern, or Identifier
        if ch.is_ascii_alphabetic() {
            return self.tok_def_extern_or_ident();
            // Number
        } else if ch.is_ascii_digit() || ch == '.' {
            return self.tok_number();
            // Comment
        } else if ch == '#' {
            return self.tok_comment();
        }

        self.try_get_char(false);
        Token::Misc(ch).into()
    }

    fn try_get_char(&mut self, does_eat_whitespace: bool) -> Option<char> {
        self.char_buffer = None;

        loop {
            // TODO: Improve error handling here
            if read_exact!(self.reader, self.byte_buffer).is_err() {
                return None;
            }

            if self.byte_buffer[0].is_ascii() {
                self.char_buffer = char::from(self.byte_buffer[0]).into();
            } else {
                eprintln!(
                    "Read non-ASCII byte '{}' while Lexing, exiting...\n",
                    self.byte_buffer[0]
                );
                std::process::exit(1);
            }

            match self.char_buffer {
                Some(c) => {
                    if does_eat_whitespace && c.is_ascii_whitespace() {
                    } else {
                        return Some(c);
                    }
                }
                None => return None,
            }
        }
    }

    fn tok_number(&mut self) -> Option<Token> {
        let mut ch = self.char_buffer.unwrap();
        let mut saw_decimal = ch == '.';
        let mut num_string = String::new();

        loop {
            num_string.push(ch);
            match self.try_get_char(false) {
                Some(c) => ch = c,
                None => break,
            }

            // If we already have a decimal in the number, and this is a decimal,
            // we can't read any more digits => bail.
            if saw_decimal && ch == '.' {
                break;
            }
            saw_decimal = ch == '.';

            if !ch.is_ascii_digit() && ch != '.' {
                break;
            }
        }

        let num_val = num_string.parse::<f64>().ok()?;
        Token::Number(num_val).into()
    }

    fn tok_comment(&mut self) -> Option<Token> {
        loop {
            // Read until a newline character
            let result = self.try_get_char(false);

            match result {
                // get_token ignores whitespace, so we'll eat whitespaces until we get a token.
                opt if Self::is_newline(opt) => return self.get_token(),
                Some(_) => (),
                None => return Token::EOF.into(),
            }
        }
    }

    fn tok_def_extern_or_ident(&mut self) -> Option<Token> {
        let mut ident = String::new();
        let mut ch = self.char_buffer.unwrap();

        while ch.is_alphanumeric() {
            ident.push(ch);
            match self.try_get_char(false) {
                Some(c) => ch = c,
                None => break,
            }
        }

        match ident.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            _ => Token::Identifier(ident),
        }
        .into()
    }
}

#[cfg(test)]
mod tests;
