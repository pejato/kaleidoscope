use std::{
    io::{BufRead, ErrorKind},
    string::String,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Misc(char),
}

pub struct Lexer<T>
where
    T: BufRead,
{
    reader: T,
    buffer: Option<Token>,
    char_buffer: Option<char>,
    byte_buffer: [u8; 1],
}

// Public Interface

impl<T> Lexer<T>
where
    T: BufRead,
{
    pub fn new(reader: T) -> Self {
        Lexer {
            reader: reader,
            buffer: None,
            char_buffer: None,
            byte_buffer: [0],
        }
    }

    pub fn get_next_token(&mut self) {
        self.buffer = self.get_token().into();
    }

    pub fn current_token(&self) -> &Option<Token> {
        &self.buffer
    }
}

// Private methods

#[macro_export]
macro_rules! read_exact {
    ($reader: expr, $buf:expr) => {{
        match $reader.read_exact(&mut $buf) {
            Ok(_) => (),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                eprintln!("Got EOF, exiting...\n");
                std::process::exit(0);
            }
            Err(_) => {
                eprintln!("Failed to read character while Lexing, exiting...\n");
                std::process::exit(1);
            }
        }
    };};
}

impl<T> Lexer<T>
where
    T: BufRead,
{
    fn is_newline(c: Option<char>) -> bool {
        if c.is_none() {
            return false;
        }
        let c = c.unwrap();

        c == '\n' || c == '\r'
    }

    // Methods

    fn try_get_char(&mut self, does_eat_whitespace: bool) -> char {
        // TODO: Figure out if we can detect if input stream is not unicode
        // TODO: Pass a closure here to determine when to return a char
        loop {
            read_exact!(self.reader, self.byte_buffer);

            if self.byte_buffer[0].is_ascii() {
                self.char_buffer = char::from(self.byte_buffer[0]).into();
            }

            match self.char_buffer {
                Some(c) if !does_eat_whitespace && c.is_ascii_whitespace() => (),
                Some(c) => {
                    return c;
                }
                None => {
                    eprintln!(
                        "Read non-ASCII byte '{}' while Lexing, exiting...\n",
                        self.byte_buffer[0]
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    fn get_token(&mut self) -> Token {
        let ch: char;

        // Check if there's a non-whitespace char already in the buffer
        if self.char_buffer.map_or(true, |c| c.is_ascii_whitespace()) {
            ch = self.try_get_char(true);
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

        Token::Misc(ch)
    }

    fn tok_number(&mut self) -> Token {
        let mut ch = self.char_buffer.unwrap();
        let mut saw_decimal = ch == '.';
        let mut num_string = String::new();

        loop {
            num_string.push(ch);
            ch = self.try_get_char(false);

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

        let num_val = num_string.parse::<f64>().unwrap();
        return Token::Number(num_val);
    }

    fn tok_comment(&mut self) -> Token {
        loop {
            // Read until a newline character
            let ch = self.try_get_char(false);

            if Self::is_newline(ch.into()) {
                // This ignores whitespace, so we'll eat all whitespaces until we get
                // a token.
                return self.get_token();
            }
        }
    }

    // fix calling convention wrt other functions. Either pass the first char or don't
    fn tok_def_extern_or_ident(&mut self) -> Token {
        let mut ident = String::new();
        let mut ch = self.char_buffer.unwrap();

        while ch.is_alphanumeric() {
            ident.push(ch);
            ch = self.try_get_char(false);
        }

        return match ident.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Identifier(ident),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::test::approx_equal;

    use super::*;

    #[test]
    fn test_tok_number_valid_integer() {
        let mut lexer = Lexer::new("123456789".as_bytes());
        let result = lexer.tok_number();

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.0, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_number_valid_decimal() {
        let mut lexer = Lexer::new("123456789.3798901".as_bytes());
        let result = lexer.tok_number();

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.3798901, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic(
        // TODO: We really shouldn't be panicking in this situation.
        expected = "called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }"
    )]
    fn test_tok_number_too_many_decimal_points() {
        let mut lexer = Lexer::new("123456789.37989.01".as_bytes());
        let _ = lexer.tok_number();
    }

    #[test]
    fn test_tok_valid_def() {
        let mut lexer = Lexer::new("def".as_bytes());
        let result = lexer.tok_def_extern_or_ident();

        match result {
            Token::Def => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_valid_extern() {
        let mut lexer = Lexer::new("extern".as_bytes());
        let result = lexer.tok_def_extern_or_ident();

        match result {
            Token::Extern => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_comment_with_newline_then_eof() {
        let mut lexer = Lexer::new("# Some text like def extern\n".as_bytes());
        let result = lexer.tok_comment();

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_comment_with_no_newline_then_eof() {
        let mut lexer = Lexer::new("# Some text like def extern".as_bytes());
        let result = lexer.tok_comment();

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_valid_integer() {
        let mut lexer = Lexer::new("123456789".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.0, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_valid_decimal() {
        let mut lexer = Lexer::new("123456789.3798901".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.3798901, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }"
    )]
    fn test_get_token_too_many_decimal_points() {
        let mut lexer = Lexer::new("123456789.37989.01".as_bytes());
        let _ = lexer.get_token();
    }

    #[test]
    fn test_get_token_def() {
        let mut lexer = Lexer::new("def".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::Def => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_extern() {
        let mut lexer = Lexer::new("extern".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::Extern => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_with_comment_newline_then_eof() {
        let mut lexer = Lexer::new("# Some text like def extern\n".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_with_comment_no_newline_then_eof() {
        let mut lexer = Lexer::new("# Some text like def extern".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_alpha_ident() {
        let mut lexer = Lexer::new("someident".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::Identifier(s) => assert_eq!(s, "someident".to_string()),
            _ => assert!(false, "Expected Identifier but got {:?}", result),
        }
    }

    #[test]
    fn test_get_token_alphanumeric_ident() {
        let mut lexer = Lexer::new("someident78".as_bytes());
        let result = lexer.get_token();

        match result {
            Token::Identifier(s) => assert_eq!(s, "someident78".to_string()),
            _ => assert!(false, "Expected Identifier but got {:?}", result),
        }
    }

    #[test]
    fn test_get_token_integration_all_tokens() {
        let mut lexer = Lexer::new("def extern someident3 77.03 + # some stuff\n\ry".as_bytes());
        let mut result = lexer.get_token();

        match result {
            Token::Def => (),
            _ => assert!(false, "Expected Def but got {:?}", result),
        }

        result = lexer.get_token();
        match result {
            Token::Extern => (),
            _ => assert!(false, "Expected Extern but got {:?}", result),
        }

        result = lexer.get_token();
        match result {
            Token::Identifier(s) => assert_eq!(s, "someident3".to_string()),
            _ => assert!(
                false,
                "Expected {:?} but got {:?}",
                Token::Identifier("someident3".to_string()),
                result
            ),
        }

        result = lexer.get_token();
        match result {
            Token::Number(n) => assert!(
                approx_equal(n, 77.03, 8),
                "Expected {:?} but got {:?}",
                Token::Number(77.03),
                n
            ),
            _ => assert!(
                false,
                "Expected {:?} but got {:?}",
                Token::Number(77.03),
                result
            ),
        }

        result = lexer.get_token();
        match result {
            Token::Misc(c) => assert_eq!(c, '+'),
            _ => assert!(
                false,
                "Expected {:?} but got {:?}",
                Token::Misc('+'),
                result
            ),
        }

        result = lexer.get_token();
        match result {
            Token::EOF => (),
            _ => assert!(false, "Expected {:?} but got {:?}", Token::EOF, result),
        }
    }
}
