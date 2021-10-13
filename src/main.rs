use std::{io::Read, string::String};

// TODO: Operators
enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}

fn is_decimal(c: u8) -> bool {
    c == ('.' as u8)
}

fn is_newline(c: u8) -> bool {
    c == ('\n' as u8) || c == ('\r' as u8)
}

// TODO: Look at actual error type in stdin().read
// TODO: Break this up
fn get_token() -> Token {
    let mut char: [u8; 1] = [0];
    let mut stdin = std::io::stdin();

    loop {
        if let Err(_) = stdin.read(&mut char) {
            return Token::EOF;
        }
        if !char[0].is_ascii_whitespace() {
            break;
        }
    }

    // Def, Extern, or Identifier
    if char[0].is_ascii_alphabetic() {
        let mut ident = (char[0] as char).to_string();

        while char[0].is_ascii_alphanumeric() {
            if let Err(_) = stdin.read(&mut char) {
                break;
            }
            ident.push(char[0] as char);
        }

        return match ident.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Identifier(ident),
        };
    // Number
    } else if char[0].is_ascii_digit() || is_decimal(char[0]) {
        let mut saw_decimal = is_decimal(char[0]);
        let mut num_string = (char[0] as char).to_string();

        while char[0].is_ascii_digit() || is_decimal(char[0]) {
            if let Err(_) = stdin.read(&mut char) {
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
        // Comment
    } else if char[0] == ('#' as u8) {
        loop {
            // Read until EOF or a newline character
            if let Err(_) = stdin.read(&mut char) {
                return Token::EOF;
            }
            if is_newline(char[0]) {
                // Strip characters until we encounter a non-newline
                while is_newline(char[0]) {
                    if let Err(_) = stdin.read(&mut char) {
                        return Token::EOF;
                    }
                }
                return get_token();
            }
        }
    }

    Token::EOF
}

fn main() {
    let my_str = "1.23.4".to_string();

    println!("{}", my_str.parse::<f64>().unwrap());
    println!("Hello, world!");
}
