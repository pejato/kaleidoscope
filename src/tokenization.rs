use std::{io::Read, string::String};

// TODO: Operators?
enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Misc(char),
}

fn is_decimal(c: u8) -> bool {
    c == ('.' as u8)
}

fn is_newline(c: u8) -> bool {
    c == ('\n' as u8) || c == ('\r' as u8)
}

// TODO: Look at actual error type in reader.read
fn get_token<T>(mut reader: T) -> Token
where
    T: Read,
{
    let mut char: [u8; 1] = [0];
    let mut n: usize;

    loop {
        n = reader.read(&mut char).unwrap();
        if n == 0 {
            break;
        }
        if !char[0].is_ascii_whitespace() {
            break;
        }
    }

    // Def, Extern, or Identifier
    if char[0].is_ascii_alphabetic() {
        return tok_def_extern_or_ident(char, reader);
        // Number
    } else if char[0].is_ascii_digit() || is_decimal(char[0]) {
        return tok_number(char, reader);
        // Comment
    } else if char[0] == ('#' as u8) {
        return tok_comment(char, reader);
    }

    if n == 0 {
        Token::EOF
    } else {
        Token::Misc(char[0] as char)
    }
}

fn tok_number<T>(mut char: [u8; 1], mut reader: T) -> Token
where
    T: Read,
{
    let mut saw_decimal = is_decimal(char[0]);
    let mut num_string = (char[0] as char).to_string();

    loop {
        let n = reader.read(&mut char).unwrap();
        if n == 0 {
            break;
        }

        // If we already have a decimal in the number, and this is a decimal, we can't read any more digits => bail.
        if saw_decimal && is_decimal(char[0]) {
            break;
        }
        saw_decimal = is_decimal(char[0]);
        num_string.push(char[0] as char);

        if !char[0].is_ascii_digit() && !is_decimal(char[0]) {
            break;
        }
    }

    let num_val = num_string.parse::<f64>().unwrap();
    return Token::Number(num_val);
}

fn tok_comment<T>(mut char: [u8; 1], mut reader: T) -> Token
where
    T: Read,
{
    loop {
        // Read until EOF or a newline character
        let n = reader.read(&mut char).unwrap();
        if n == 0 {
            return Token::EOF;
        }
        if is_newline(char[0]) {
            // Strip characters until we encounter a non-newline
            while is_newline(char[0]) {
                match reader.read(&mut char) {
                    Err(_) | Ok(0) => return Token::EOF,
                    _ => (),
                }
            }
            return get_token(reader);
        }
    }
}

fn tok_def_extern_or_ident<T>(mut char: [u8; 1], mut reader: T) -> Token
where
    T: Read,
{
    let mut ident = (char[0] as char).to_string();

    while char[0].is_ascii_alphanumeric() {
        let n = reader.read(&mut char).unwrap();
        if n == 0 {
            break;
        }
        ident.push(char[0] as char);
    }

    return match ident.as_str() {
        "def" => Token::Def,
        "extern" => Token::Extern,
        _ => Token::Identifier(ident),
    };
}
#[cfg(test)]
mod tests {
    use crate::test_utilities::approx_equal;

    use super::*;

    #[test]
    fn test_tok_number_valid_integer() {
        let mut buf: [u8; 1] = [0];
        let mut input = "123456789".as_bytes();
        let _ = input.read(&mut buf);

        let result = tok_number(buf, input);

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.0, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_number_valid_decimal() {
        let mut buf: [u8; 1] = [0];
        let mut input = "123456789.3798901".as_bytes();
        let _ = input.read(&mut buf);

        let result = tok_number(buf, input);

        match result {
            Token::Number(n) => assert!(crate::test_utilities::approx_equal(
                n,
                123456789.3798901,
                15
            )),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }"
    )]
    fn test_tok_number_too_many_decimal_points() {
        let mut buf: [u8; 1] = [0];
        let mut input = "123456789.37989.01".as_bytes();
        let _ = input.read(&mut buf);

        let _ = tok_number(buf, input);
    }

    #[test]
    fn test_tok_valid_def() {
        let mut buf: [u8; 1] = [0];
        let mut input = "def".as_bytes();
        let _ = input.read(&mut buf);

        let result = tok_def_extern_or_ident(buf, input);

        match result {
            Token::Def => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_valid_extern() {
        let mut buf: [u8; 1] = [0];
        let mut input = "extern".as_bytes();
        let _ = input.read(&mut buf);

        let result = tok_def_extern_or_ident(buf, input);

        match result {
            Token::Extern => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_comment_with_newline_then_eof() {
        let mut buf: [u8; 1] = [0];
        let mut input = "# Some text like def extern\n".as_bytes();
        let _ = input.read(&mut buf);

        let result = tok_comment(buf, input);

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_comment_with_no_newline_then_eof() {
        let mut buf: [u8; 1] = [0];
        let mut input = "# Some text like def extern".as_bytes();
        let _ = input.read(&mut buf);

        let result = tok_comment(buf, input);

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_valid_integer() {
        let input = "123456789".as_bytes();
        let result = get_token(input);

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.0, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_valid_decimal() {
        let input = "123456789.3798901".as_bytes();
        let result = get_token(input);

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
        let input = "123456789.37989.01".as_bytes();
        let _ = get_token(input);
    }

    #[test]
    fn test_get_token_def() {
        let input = "def".as_bytes();
        let result = get_token(input);

        match result {
            Token::Def => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_extern() {
        let input = "extern".as_bytes();
        let result = get_token(input);

        match result {
            Token::Extern => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_with_comment_newline_then_eof() {
        let input = "# Some text like def extern\n".as_bytes();
        let result = get_token(input);

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_token_with_comment_no_newline_then_eof() {
        let input = "# Some text like def extern".as_bytes();
        let result = get_token(input);

        match result {
            Token::EOF => (),
            _ => assert!(false),
        }
    }
}
