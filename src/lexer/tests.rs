use crate::lexer::Token::*;
use crate::test_utilities::test::approx_equal;
use pretty_assertions::assert_eq;

use super::*;

#[test]
fn test_tok_number_valid_integer() {
    let mut lexer = Lexer::new("23456789".as_bytes());
    lexer.char_buffer = '1'.into();
    let result = lexer.tok_number();

    match result {
        Some(Number(n)) => assert!(approx_equal(n, 123456789.0, 15)),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_tok_number_valid_decimal() {
    let mut lexer = Lexer::new("23456789.3798901".as_bytes());
    lexer.char_buffer = '1'.into();
    let result = lexer.tok_number();

    match result {
        Some(Number(n)) => assert!(approx_equal(n, 123456789.3798901, 15)),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_tok_number_too_many_decimal_points() {
    let mut lexer = Lexer::new("23456789.37989.01".as_bytes());
    lexer.char_buffer = '1'.into();
    let result = lexer.tok_number();
    assert!(result.is_none());
}

#[test]
fn test_tok_valid_def() {
    let mut lexer = Lexer::new("ef".as_bytes());
    lexer.char_buffer = 'd'.into();
    let result = lexer.tok_def_extern_or_ident();

    match result {
        Some(Def) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_tok_valid_extern() {
    let mut lexer = Lexer::new("xtern".as_bytes());
    lexer.char_buffer = 'e'.into();
    let result = lexer.tok_def_extern_or_ident();

    match result {
        Some(Extern) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_tok_comment_with_newline_then_eof() {
    let mut lexer = Lexer::new(" Some text like def extern\n".as_bytes());
    lexer.char_buffer = '#'.into();
    let result = lexer.tok_comment();

    match result {
        Some(EOF) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_tok_comment_with_no_newline_then_eof() {
    let mut lexer = Lexer::new("# Some text like def extern".as_bytes());
    let result = lexer.tok_comment();

    match result {
        Some(EOF) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_valid_integer() {
    let mut lexer = Lexer::new("123456789".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(Number(n)) => assert!(approx_equal(n, 123456789.0, 15)),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_valid_decimal() {
    let mut lexer = Lexer::new("123456789.3798901".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(Number(n)) => assert!(approx_equal(n, 123456789.3798901, 15)),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_too_many_decimal_points() {
    let mut lexer = Lexer::new("123456789.37989.01".as_bytes());
    let result = lexer.get_token();
    assert!(result.is_none());
}

#[test]
fn test_get_token_def() {
    let mut lexer = Lexer::new("def".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(Def) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_extern() {
    let mut lexer = Lexer::new("extern".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(Extern) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_with_comment_newline_then_eof() {
    let mut lexer = Lexer::new("# Some text like def extern\n".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(EOF) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_with_comment_no_newline_then_eof() {
    let mut lexer = Lexer::new("# Some text like def extern".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(EOF) => (),
        _ => assert!(false, "{:#?}", result),
    }
}

#[test]
fn test_get_token_alpha_ident() {
    let mut lexer = Lexer::new("someident".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(Identifier(s)) => assert_eq!(s, "someident".to_string()),
        _ => assert!(false, "Expected Identifier but got {:?}", result),
    }
}

#[test]
fn test_get_token_alphanumeric_ident() {
    let mut lexer = Lexer::new("someident78".as_bytes());
    let result = lexer.get_token();

    match result {
        Some(Identifier(s)) => assert_eq!(s, "someident78".to_string()),
        _ => assert!(false, "Expected Identifier but got {:?}", result),
    }
}

#[test]
fn test_get_token_integration_all_tokens() {
    let mut lexer = Lexer::new("def extern someident3 77.03 + # some stuff\n\ry".as_bytes());
    let mut result = lexer.get_token();

    match result {
        Some(Def) => (),
        _ => assert!(false, "Expected Def but got {:?}", result),
    }

    result = lexer.get_token();
    match result {
        Some(Extern) => (),
        _ => assert!(false, "Expected Extern but got {:?}", result),
    }

    result = lexer.get_token();
    match result {
        Some(Identifier(s)) => assert_eq!(s, "someident3".to_string()),
        _ => assert!(
            false,
            "Expected {:?} but got {:?}",
            Identifier("someident3".to_string()),
            result
        ),
    }

    result = lexer.get_token();
    match result {
        Some(Number(n)) => assert!(
            approx_equal(n, 77.03, 8),
            "Expected {:?} but got {:?}",
            Number(77.03),
            n
        ),
        _ => assert!(false, "Expected {:?} but got {:?}", Number(77.03), result),
    }

    result = lexer.get_token();
    match result {
        Some(Misc(c)) => assert_eq!(c, '+'),
        _ => assert!(false, "Expected {:?} but got {:?}", Misc('+'), result),
    }

    result = lexer.get_token();
    match result {
        Some(Identifier(s)) if s == *"y" => (),
        _ => assert!(
            false,
            "Expected {:?} but got {:?}",
            Identifier("y".into()),
            result
        ),
    }

    result = lexer.get_token();
    match result {
        Some(EOF) => (),
        _ => assert!(false, "Expected {:?} but got {:?}", EOF, result),
    }
}

#[test]
fn test_lex_if_then_else() {
    let mut lexer = Lexer::new("if then else".as_bytes());
    assert_eq!(lexer.get_next_token(), &Token::If.into());
    assert_eq!(lexer.get_next_token(), &Token::Then.into());
    assert_eq!(lexer.get_next_token(), &Token::Else.into());
}
