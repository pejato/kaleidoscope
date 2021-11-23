use std::str::FromStr;

use super::*;
use crate::parser::ExprKind::*;
use crate::test_utilities::test::approx_equal;
use pretty_assertions::assert_eq;

#[macro_export]
macro_rules! setup_parser_lexer {
    ($input_string: literal) => {
        (Parser::new(), {
            let mut lexer = Lexer::new($input_string.as_bytes());
            lexer.get_next_token();
            lexer
        })
    };
}

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
    let (mut parser, mut lexer) = setup_parser_lexer!("64 + 3");

    let result = parser.parse_number_expr(&mut lexer);

    match result {
        Expr { kind: Number(val) } => assert!(approx_equal(64.0, val, 5)),
        _ => assert!(false, "Expected ExprKind::Number"),
    }
}

#[test]
fn test_parse_number_expr_consumes_token() {
    let (mut parser, mut lexer) = setup_parser_lexer!("64 + 3");

    let current_token: Option<Token> = lexer.current_token().clone();

    match current_token {
        Some(Token::Number(num)) => {
            parser.parse_number_expr(&mut lexer);
            assert!(approx_equal(64.0, num, 5))
        }
        _ => assert!(false, "Expected Token::Number(64.0)"),
    }

    match lexer.current_token() {
        Some(Token::Misc('+')) => (),
        _ => assert!(false, "Expected '+'"),
    }
}

#[test]
fn test_parse_paren_expr() {
    let (mut parser, mut lexer) = setup_parser_lexer!("(78)");

    let result = parser.parse_paren_expr(&mut lexer);

    match result {
        Some(Expr { kind: Number(val) }) => assert!(approx_equal(val, 78.0, 5)),
        _ => assert!(false, "Expected Expr::Kind(Number(78))"),
    }
}

#[test]
fn test_parse_identifier_prefixed_expr_parses_variable() {
    let (mut parser, mut lexer) = setup_parser_lexer!("ident42");

    let result = parser.parse_identifier_prefixed_expr("ident42".into(), &mut lexer);
    let expected_value = Expr {
        kind: Variable {
            name: "ident42".into(),
        },
    };
    match result {
        Some(expr) if expr == expected_value => (),
        _ => assert!(false, "Expected {:#?}", expected_value),
    }
}

#[test]
fn test_parse_identifier_prefixed_expr_parses_call() {
    let (mut parser, mut lexer) = setup_parser_lexer!("ident42(30)");

    let result = parser.parse_identifier_prefixed_expr("ident42".into(), &mut lexer);
    let expected_value = Expr {
        kind: Call {
            args: vec![Expr { kind: Number(30.0) }],
            callee: "ident42".into(),
        },
    };
    match result {
        // TODO: Not a great thing to be relying on equality of f64...
        Some(expr) if expr == expected_value => (),
        _ => assert!(false, "Expected {:#?}", expected_value),
    }
}

#[test]
fn test_parse_identifier_prefixed_expr_parsed_call_multiple_args() {
    let (mut parser, mut lexer) = setup_parser_lexer!("ident66(30, 60, 90)");

    let result = parser.parse_identifier_prefixed_expr("ident42".into(), &mut lexer);
    let expected_value = Expr {
        kind: Call {
            args: vec![
                Expr { kind: Number(30.0) },
                Expr { kind: Number(60.0) },
                Expr { kind: Number(90.0) },
            ],
            callee: "ident42".into(),
        },
    };
    match result {
        Some(expr) if expr == expected_value => (),
        _ => assert!(false, "Expected {:#?}", expected_value),
    }
}

#[test]
fn test_parse_primary_expr_parses_number() {
    let (mut parser, mut lexer) = setup_parser_lexer!("657");

    let result = parser.parse_primary_expr(&mut lexer);

    assert_eq!(
        result,
        Expr {
            kind: Number(657.0)
        }
        .into()
    );
}

#[test]
fn test_parse_primary_expr_parses_ident_prefix_into_variable() {
    let (mut parser, mut lexer) = setup_parser_lexer!("suwooooo");

    let result = parser.parse_primary_expr(&mut lexer);
    assert_eq!(
        result,
        Expr {
            kind: Variable {
                name: "suwooooo".into()
            }
        }
        .into()
    );
}

#[test]
fn test_parse_primary_expr_parses_ident_prefix_into_call() {
    let (mut parser, mut lexer) = setup_parser_lexer!("suwooooo()");

    let result = parser.parse_primary_expr(&mut lexer);
    assert_eq!(
        result,
        Expr {
            kind: Call {
                args: vec![],
                callee: "suwooooo".into()
            }
        }
        .into()
    );
}

#[test]
fn test_parse_primary_expr_parses_paren_expr() {
    let (mut parser, mut lexer) = setup_parser_lexer!("(5 + yar())");

    let result = parser.parse_primary_expr(&mut lexer);
    let expected_result = Expr {
        kind: Binary {
            lhs: Expr { kind: Number(5.0) }.into(),
            rhs: Expr {
                kind: Call {
                    callee: "yar".into(),
                    args: vec![],
                },
            }
            .into(),
            operator: '+',
        },
    }
    .into();

    assert_eq!(result, expected_result);
}

// TODO: Pass write stream to use where we use eprintln! currently
#[test]
fn test_parse_primary_expr_logs_error() {
    let (mut parser, mut lexer) = setup_parser_lexer!("def");

    let result = parser.parse_primary_expr(&mut lexer);
    assert_eq!(result, None);
}

#[test]
fn test_parse_multiple_ops() {
    let (mut parser, mut lexer) = setup_parser_lexer!("3 + 2 - 4 * 7 < 3");

    let result = parser.parse_expression(&mut lexer);
    // This is a mess to look at, but it represents (3 + (2 - (4 * 7))) < 3
    let expected_result = Expr {
        kind: Binary {
            operator: '<',
            lhs: Expr {
                kind: Binary {
                    operator: '+',
                    lhs: Expr { kind: Number(3.0) }.into(),
                    rhs: Expr {
                        kind: Binary {
                            operator: '-',
                            lhs: Expr { kind: Number(2.0) }.into(),
                            rhs: Expr {
                                kind: Binary {
                                    operator: '*',
                                    lhs: Expr { kind: Number(4.0) }.into(),
                                    rhs: Expr { kind: Number(7.0) }.into(),
                                },
                            }
                            .into(),
                        }
                        .into(),
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
            rhs: Expr { kind: Number(3.0) }.into(),
        },
    }
    .into();

    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_multiple_add_ops() {
    let (mut parser, mut lexer) = setup_parser_lexer!("1+2+3+4");

    let result = parser.parse_expression(&mut lexer);
    let expected_result = Expr {
        kind: Binary {
            operator: '+',
            lhs: Expr {
                kind: Binary {
                    operator: '+',
                    lhs: Expr {
                        kind: Binary {
                            operator: '+',
                            lhs: Expr { kind: Number(1.0) }.into(),
                            rhs: Expr { kind: Number(2.0) }.into(),
                        },
                    }
                    .into(),
                    rhs: Expr { kind: Number(3.0) }.into(),
                },
            }
            .into(),
            rhs: Expr { kind: Number(4.0) }.into(),
        },
    }
    .into();

    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_function_proto_legal() {
    let (mut parser, mut lexer) = setup_parser_lexer!("fn(three, four, five)");

    let result = parser.parse_function_prototype(&mut lexer);
    let expected_result = Expr {
        kind: Prototype {
            name: "fn".to_owned(),
            args: vec![
                String::from_str("three").unwrap(),
                String::from_str("four").unwrap(),
                String::from_str("five").unwrap(),
            ],
        },
    }
    .into();

    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_function_proto_legal_no_args() {
    let (mut parser, mut lexer) = setup_parser_lexer!("fn()");

    let result = parser.parse_function_prototype(&mut lexer);
    let expected_result = Expr {
        kind: Prototype {
            name: "fn".to_owned(),
            args: vec![],
        },
    }
    .into();

    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_function_proto_legal_trailing_comma_after_args() {
    let (mut parser, mut lexer) = setup_parser_lexer!("fn(seven,)");

    let result = parser.parse_function_prototype(&mut lexer);
    let expected_result = Expr {
        kind: Prototype {
            name: "fn".to_owned(),
            args: vec![String::from_str("seven").unwrap()],
        },
    }
    .into();

    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_function_proto_illegal_no_fn_name() {
    let (mut parser, mut lexer) = setup_parser_lexer!("(three, four, five)");

    let result = parser.parse_function_prototype(&mut lexer);
    let expected_result = None;

    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_function_proto_illegal_no_opening_brace() {
    let (mut parser, mut lexer) = setup_parser_lexer!("fUNKthree, four, five)");

    let result = parser.parse_function_prototype(&mut lexer);
    let expected_result = None;
    assert_eq!(result, expected_result);
}

#[test]
fn test_parse_function_proto_illegal_no_closing_brace() {
    let (mut parser, mut lexer) = setup_parser_lexer!("FuN(three, four, five");

    let result = parser.parse_function_prototype(&mut lexer);
    let expected_result = None;

    assert_eq!(result, expected_result);
}
