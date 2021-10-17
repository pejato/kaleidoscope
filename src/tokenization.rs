use crate::test_utilities::approx_equal;

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

// TODO: Look at actual error type in stdin().read
fn get_token() -> Token {
    let mut char: [u8; 1] = [0];
    let mut stdin = std::io::stdin();
    let mut n: usize;

    loop {
        n = stdin.read(&mut char).unwrap();
        if n == 0 {
            break;
        }
        if !char[0].is_ascii_whitespace() {
            break;
        }
    }

    // Def, Extern, or Identifier
    if char[0].is_ascii_alphabetic() {
        return tok_def_extern_or_ident(char, stdin);
        // Number
    } else if char[0].is_ascii_digit() || is_decimal(char[0]) {
        return tok_number(char, stdin);
        // Comment
    } else if char[0] == ('#' as u8) {
        return tok_comment(char, stdin);
    }

    if n == 0 {
        Token::EOF
    } else {
        Token::Misc(char[0] as char)
    }
}

fn tok_number<T>(mut char: [u8; 1], mut stdin: T) -> Token
where
    T: Read,
{
    let mut saw_decimal = is_decimal(char[0]);
    let mut num_string = (char[0] as char).to_string();

    loop {
        let n = stdin.read(&mut char).unwrap();
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

fn tok_comment<T>(mut char: [u8; 1], mut stdin: T) -> Token
where
    T: Read,
{
    loop {
        // Read until EOF or a newline character
        let n = stdin.read(&mut char).unwrap();
        if n == 0 {
            return Token::EOF;
        }
        if is_newline(char[0]) {
            // Strip characters until we encounter a non-newline
            while is_newline(char[0]) {
                match stdin.read(&mut char) {
                    Err(_) | Ok(0) => return Token::EOF,
                    _ => (),
                }
            }
            return get_token();
        }
    }
}

fn tok_def_extern_or_ident<T>(mut char: [u8; 1], mut stdin: T) -> Token
where
    T: Read,
{
    let mut ident = (char[0] as char).to_string();

    while char[0].is_ascii_alphanumeric() {
        let n = stdin.read(&mut char).unwrap();
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
    use super::*;

    #[test]
    fn test_tok_number_valid_integer() {
        let input = "23456789".as_bytes();
        let buf: [u8; 1] = ['1' as u8];
        let result = tok_number(buf, input);

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.0, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_tok_number_valid_decimal() {
        let input = "23456789.3798901".as_bytes();
        let buf: [u8; 1] = ['1' as u8];
        let result = tok_number(buf, input);

        match result {
            Token::Number(n) => assert!(approx_equal(n, 123456789.3798901, 15)),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }"
    )]
    fn test_tok_number_too_many_decimal_points() {
        let input = "23456789.37989.01".as_bytes();
        let buf: [u8; 1] = ['1' as u8];
        let result = tok_number(buf, input);
    }

    #[test]
    fn test_tok_valid_def() {
        let input = "ef".as_bytes();
        let buf: [u8; 1] = ['d' as u8];
        let result = tok_def_extern_or_ident(buf, input);

        match result {
            Token::Def => (),
            _ => assert!(false),
        }
    }
}
