use std::io::BufRead;

use crate::ast::{Expr, ExprKind};
use crate::environment::Environment;
use crate::tokenization::{Lexer, Token};

pub struct Parser {
    pub environment: Environment,
}

impl Parser {
    pub fn new() -> Parser {
        let mut environment = Environment::new();
        [('<', 10), ('+', 20), ('-', 30), ('*', 40)]
            .iter()
            .for_each(|p| environment.add_operator_precedence(*p));

        Parser { environment }
    }

    // Primary expression parsing
    pub fn parse_number_expr<T: BufRead>(&mut self, value: f64, consumer: &mut Lexer<T>) -> Expr {
        let result = Expr {
            kind: ExprKind::Number { value },
        };
        consumer.get_next_token();
        return result;
    }

    pub fn parse_paren_expr<T: BufRead>(&mut self, consumer: &mut Lexer<T>) -> Option<Expr> {
        // Eat '('
        consumer.get_next_token();
        let result = self.parse_expression(consumer)?;

        match consumer.current_token() {
            Some(Token::Misc(')')) => (),
            Some(Token::Misc(c)) => return self.log_error(format!("Expected ')' but got {}!", c)),
            Some(tok) => return self.log_error(format!("Expected ')' but got {:#?}", tok)),
            None => return self.log_error("Expected ')' but got None!".into()),
        }
        // Eat ')'
        consumer.get_next_token();

        return result.into();
    }

    pub fn parse_identifier_prefixed_expr<T: BufRead>(
        &mut self,
        identifier: String,
        consumer: &mut Lexer<T>,
    ) -> Option<Expr> {
        // Eat the identifier
        consumer.get_next_token();

        match consumer.current_token() {
            Some(Token::Misc('(')) => consumer.get_next_token(),
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
            let expr = self.parse_expression(consumer)?;
            call_args.push(expr);

            // Call arguments must be postfixed by a closing parenthese or a comma
            match consumer.current_token() {
                Some(Token::Misc(')')) => break,
                Some(Token::Misc(',')) => (),
                _ => return self.log_error("Expected ')' or ','".into()),
            };
            consumer.get_next_token();
        }

        // Eat the closing parenthese
        consumer.get_next_token();

        let kind = ExprKind::Call {
            callee: identifier,
            args: call_args,
        };

        return Expr { kind }.into();
    }

    pub fn parse_primary_expr<T: BufRead>(&mut self, consumer: &mut Lexer<T>) -> Option<Expr> {
        match consumer.current_token() {
            Some(Token::Identifier(ident)) => {
                self.parse_identifier_prefixed_expr(ident.clone(), consumer)
            }
            Some(Token::Number(num)) => self.parse_number_expr(*num, consumer).into(),
            Some(Token::Misc('(')) => self.parse_paren_expr(consumer),
            _ => self.log_error("unknown token when expecting an expression".into()),
        }
    }

    // Operator parsing and precedence stuff
    pub fn parse_expression<T: BufRead>(&mut self, consumer: &mut Lexer<T>) -> Option<Expr> {
        let primary = self.parse_primary_expr(consumer)?;
        self.parse_binary_op_rhs(0, primary, consumer)
    }

    pub fn parse_binary_op_rhs<T: BufRead>(
        &mut self,
        lowest_edible_op_precedence: i32,
        mut lhs: Expr,
        consumer: &mut Lexer<T>,
    ) -> Option<Expr> {
        loop {
            // Try looking up precedence and default to -1 (which is worst than
            // any precedence) if this fails
            let (precedence, op): (i32, Option<char>) = match consumer.current_token() {
                Some(Token::Misc(c)) if c.is_ascii_alphanumeric() => (
                    self.environment.get_operator_precedence(*c).unwrap_or(-1),
                    Some(*c),
                ),
                _ => (-1, None),
            };

            // Checking if precedence is high enough priority to eat
            if lowest_edible_op_precedence > precedence {
                return lhs.into();
            }

            let op = op.unwrap();
            consumer.get_next_token();
            let mut rhs = self.parse_primary_expr(consumer)?;

            // Checking if there is a higher precedence operator to the RHS
            let next_precedence = match consumer.current_token() {
                Some(Token::Misc(c)) if c.is_ascii_alphanumeric() => {
                    self.environment.get_operator_precedence(*c).unwrap_or(-1)
                }
                _ => -1,
            };
            if next_precedence > precedence {
                // If so, recurse to the rhs
                rhs = self.parse_binary_op_rhs(precedence + 1, rhs, consumer)?;
            }
            lhs = Expr {
                kind: ExprKind::Binary {
                    operator: op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            };
        }
    }

    pub fn parse_function_prototype<T: BufRead>(
        &mut self,
        consumer: &mut Lexer<T>,
    ) -> Option<Expr> {
        let func_name: Option<String> = match consumer.current_token() {
            Some(Token::Identifier(i)) => Some(i.clone()),
            _ => None,
        };

        if func_name.is_none() {
            return self.log_error("Expected function name in protype".into());
        }

        let func_name = func_name.unwrap();
        consumer.get_next_token();

        // Opening (
        match consumer.current_token() {
            Some(Token::Misc('(')) => (),
            _ => return self.log_error("Expected '(' in prototype".into()),
        }
        consumer.get_next_token();

        let mut arg_names: Vec<String> = vec![];
        while let Some(ident) = match consumer.current_token() {
            Some(Token::Identifier(ident)) => Some(ident),
            _ => None,
        } {
            arg_names.push(ident.to_string());
            consumer.get_next_token();
        }

        match consumer.current_token() {
            Some(Token::Misc(')')) => (),
            _ => return self.log_error("Expected ')' in prototype".into()),
        }
        consumer.get_next_token();

        Expr {
            kind: ExprKind::Prototype {
                args: arg_names,
                name: func_name,
            },
        }
        .into()
    }

    pub fn parse_function_definition<T: BufRead>(
        &mut self,
        consumer: &mut Lexer<T>,
    ) -> Option<Expr> {
        // Eat 'def'
        consumer.get_next_token();
        let prototype = self.parse_function_prototype(consumer)?;
        let expression = self.parse_expression(consumer)?;

        Expr {
            kind: ExprKind::Function {
                prototype: Box::new(prototype),
                body: Box::new(expression),
            },
        }
        .into()
    }

    pub fn parse_extern<T: BufRead>(&mut self, consumer: &mut Lexer<T>) -> Option<Expr> {
        consumer.get_next_token();
        self.parse_function_prototype(consumer)
    }

    // Handle top level expressions by defining zero argument functions containing the expr
    pub fn parse_top_level_expression<T: BufRead>(
        &mut self,
        consumer: &mut Lexer<T>,
    ) -> Option<Expr> {
        let expression = self.parse_expression(consumer)?;
        let prototype = ExprKind::Prototype {
            name: "".to_string(),
            args: vec![],
        };

        Expr {
            kind: ExprKind::Function {
                prototype: Box::new(Expr { kind: prototype }),
                body: Box::new(expression),
            },
        }
        .into()
    }

    fn log_error(&self, str: String) -> Option<Expr> {
        eprintln!("log_error: {}", str);
        return None;
    }
}

#[cfg(test)]
mod tests;
