use std::{
    io::{Read, Stdin},
    string::String,
};

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

fn tok_number(mut char: [u8; 1], mut stdin: Stdin) -> Token {
    let mut saw_decimal = is_decimal(char[0]);
    let mut num_string = (char[0] as char).to_string();

    while char[0].is_ascii_digit() || is_decimal(char[0]) {
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
    }

    // Make sure this won't panic..
    let num_val = num_string.parse::<f64>().unwrap();
    return Token::Number(num_val);
}

fn tok_comment(mut char: [u8; 1], mut stdin: Stdin) -> Token {
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

fn tok_def_extern_or_ident(mut char: [u8; 1], mut stdin: Stdin) -> Token {
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
