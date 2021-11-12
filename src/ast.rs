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
fn parse_number_expr(consumer: &mut TokenConsumer) -> Option<Expr> {
    let result: Option<Expr> = match consumer.current_token() {
        Some(Token::Number(number)) => Expr {
            kind: ExprKind::Number { value: *number },
        }
        .into(),
        Some(tok) => log_error(format!("Expected Token::Number(_) but got {:#?}", tok)),
        None => log_error("Expected Token::Number(_) but got None!".into()),
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

fn parse_identifier_expr(consumer: &mut TokenConsumer) -> Option<Expr> {
    // Get the identifier string or fail out
    let identifier = match consumer.current_token() {
        Some(Token::Identifier(ident)) => Some(ident),
        _ => None,
    }?
    .clone();

    // Eat the identifier
    consumer.consume_token();

    // This is a Variable expr, not a Call expr, so we're done
    match consumer.current_token() {
        Some(Token::Misc('(')) => consumer.consume_token(),
        _ => {
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

    return Expr {
        kind: ExprKind::Call {
            callee: identifier,
            args: call_args,
        },
    }
    .into();
}

fn parse_primary_expr() -> ! {
    todo!()
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
