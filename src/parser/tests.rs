use super::*;
use crate::test_utilities::test::approx_equal;

#[test]
fn test_new_sets_up_operator_precedences() {
    let parser = Parser::new();

    assert_eq!(parser.environment.get_operator_precedence('<'), 10.into());
    assert_eq!(parser.environment.get_operator_precedence('+'), 20.into());
    assert_eq!(parser.environment.get_operator_precedence('-'), 30.into());
    assert_eq!(parser.environment.get_operator_precedence('*'), 40.into());
}

#[test]
fn test_parse_number_expr_creates_number_expr() {
    let reader = "64 + 3".as_bytes();
    let mut parser = Parser::new();
    let mut consumer = Lexer::new(reader);

    let result = parser.parse_number_expr(64.0, &mut consumer);

    match result {
        Expr {
            kind: ExprKind::Number { value: val },
        } => assert!(approx_equal(64.0, val, 5)),
        _ => assert!(false, "Expected ExprKind::Number"),
    }
}

#[test]
fn test_parse_number_expr_consumes_token() {
    let reader = "64 + 3".as_bytes();
    let mut parser = Parser::new();
    let mut consumer = Lexer::new(reader);
    consumer.get_next_token();

    let current_token: Option<Token> = consumer.current_token().clone();
    match current_token {
        Some(Token::Number(num)) => {
            parser.parse_number_expr(num, &mut consumer);
            assert!(approx_equal(64.0, num, 5))
        }
        _ => assert!(false, "Expected Token::Number(64.0)"),
    }

    match consumer.current_token() {
        Some(Token::Misc('+')) => (),
        _ => assert!(false, "Expected '+'"),
    }
}

#[test]
fn test_parse_paren_expr() {
    let reader = "(78)".as_bytes();
    let mut parser = Parser::new();
    let mut consumer = Lexer::new(reader);
    consumer.get_next_token();

    let result = parser.parse_paren_expr(&mut consumer);

    match result {
        Some(Expr {
            kind: ExprKind::Number { value: val },
        }) => assert!(approx_equal(val, 78.0, 5)),
        _ => assert!(false, "Expected Expr::Kind(Number(78))"),
    }
}
