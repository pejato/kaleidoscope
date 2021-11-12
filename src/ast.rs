use std::fmt::format;

use crate::tokenization::{Token, TokenConsumer};
struct Expr {
    kind: ExprKind,
}

enum ExprKind {
    Number {
        value: f64,
    },
    Variable {
        name: String,
    },
    Binary {
        operator: char,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Prototype {
        name: String,
        args: Vec<String>,
    },
    Function {
        proto: Box<Expr>,
        body: Vec<Expr>,
    },
}

// Primary expression parsing
fn parse_number_expr(value: f64, consumer: &mut TokenConsumer) -> Expr {
    let result = Expr {
        kind: ExprKind::Number { value },
    };
    consumer.consume_token();
    return result;
}

fn parse_paren_expr(consumer: &mut TokenConsumer) -> Option<Expr> {
    // Eat '('
    consumer.consume_token();

    let result = parse_expression();
    if result.is_none() {
        return result;
    }

    match consumer.current_token() {
        Some(Token::Misc(')')) => (),
        Some(Token::Misc(c)) => return log_error(format!("Expected ')' but got {}!", c)),
        Some(tok) => return log_error(format!("Expected ')' but got {:#?}", tok)),
        None => return log_error("Expected ')' but got None!".into()),
    }
    // Eat ')'
    consumer.consume_token();

    return result;
}

fn parse_identifier_expr(identifier: String, consumer: &mut TokenConsumer) -> Option<Expr> {
    // Eat the identifier
    consumer.consume_token();

    match consumer.current_token() {
        Some(Token::Misc('(')) => consumer.consume_token(),
        _ => {
            // This is a Variable expr, not a Call expr, so we're done
            return Expr {
                kind: ExprKind::Variable { name: identifier },
            }
            .into();
        }
    };

    // Constructing a Call expr
    let mut call_args = Vec::<Expr>::new();

    while consumer.current_token() != &Some(Token::Misc(')')) {
        // Try to parse an expr or bail
        let expr = parse_expression()?;
        call_args.push(expr);

        // Call arguments must be postfixed by a closing parenthese or a comma
        match consumer.current_token() {
            Some(Token::Misc(')')) => break,
            Some(Token::Misc(',')) => (),
            _ => return log_error("Expected ')' or ','".into()),
        };
        consumer.consume_token();
    }

    // Eat the closing parenthese
    consumer.consume_token();

    let kind = ExprKind::Call {
        callee: identifier,
        args: call_args,
    };

    return Expr { kind }.into();
}

fn parse_primary_expr(consumer: &mut TokenConsumer) -> Option<Expr> {
    match consumer.current_token() {
        Some(Token::Identifier(ident)) => parse_identifier_expr(ident.clone(), consumer),
        Some(Token::Number(num)) => parse_number_expr(*num, consumer).into(),
        Some(Token::Misc('(')) => parse_paren_expr(consumer),
        _ => log_error("unknown token when expecting an expression".into()),
    }
}

// Operator parsing and precedence stuff
fn parse_expression() -> Option<Expr> {
    todo!()
}

fn parse_binary_op_rhs() -> ! {
    todo!()
}

fn parse_function_prototype() -> ! {
    todo!()
}

fn parse_function_definition() -> ! {
    todo!()
}

fn parse_extern() -> ! {
    todo!()
}

// Handle top level expressions by defining zero argument functions containing the expr
fn parse_top_level_expression() -> ! {
    todo!()
}

fn log_error(str: String) -> Option<Expr> {
    eprintln!("log_error: {}", str);
    return None;
}
