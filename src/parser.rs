use crate::{
    ast::{Expr, ExprKind},
    environment::Environment,
    tokenization::{Lex, Token},
};

pub trait Parse {
    fn new() -> Self;
    fn parse_number_expr<L: Lex>(&mut self, lexer: &mut L) -> Expr;
    fn parse_paren_expr<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
    fn parse_identifier_prefixed_expr<L: Lex>(
        &mut self,
        identifier: String,
        lexer: &mut L,
    ) -> Option<Expr>;
    fn parse_primary_expr<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
    fn parse_expression<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
    fn parse_binary_op_rhs<L: Lex>(
        &mut self,
        lowest_edible_op_precedence: i32,
        lhs: Expr,
        lexer: &mut L,
    ) -> Option<Expr>;
    fn parse_function_prototype<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
    fn parse_function_definition<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
    fn parse_extern<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
    fn parse_top_level_expression<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr>;
}

pub struct Parser {
    pub environment: Environment,
}

impl Parser {
    fn log_error(&self, str: String) -> Option<Expr> {
        eprintln!("log_error: {}", str);
        return None;
    }
}

impl Parse for Parser {
    fn new() -> Self {
        let mut environment = Environment::new();
        [('<', 10), ('+', 20), ('-', 30), ('*', 40)]
            .iter()
            .for_each(|p| environment.add_operator_precedence(*p));

        Parser { environment }
    }

    // Primary expression parsing
    fn parse_number_expr<L: Lex>(&mut self, lexer: &mut L) -> Expr {
        let value = match lexer.current_token() {
            Some(Token::Number(v)) => *v,
            _ => unreachable!("lexer should have loaded a Number prior to calling this"),
        };
        let result = Expr {
            kind: ExprKind::Number(value),
        };
        lexer.get_next_token();
        return result;
    }

    fn parse_paren_expr<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        // Eat '('
        lexer.get_next_token();
        let result = self.parse_expression(lexer)?;

        match lexer.current_token() {
            Some(Token::Misc(')')) => (),
            Some(Token::Misc(c)) => return self.log_error(format!("Expected ')' but got {}!", c)),
            Some(tok) => return self.log_error(format!("Expected ')' but got {:#?}", tok)),
            None => return self.log_error("Expected ')' but got None!".into()),
        }
        // Eat ')'
        lexer.get_next_token();

        return result.into();
    }

    fn parse_identifier_prefixed_expr<L: Lex>(
        &mut self,
        identifier: String,
        lexer: &mut L,
    ) -> Option<Expr> {
        // Eat the identifier
        lexer.get_next_token();

        match lexer.current_token() {
            Some(Token::Misc('(')) => lexer.get_next_token(),
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

        while lexer.current_token() != &Some(Token::Misc(')')) {
            // Try to parse an expr or bail
            let expr = self.parse_expression(lexer)?;
            call_args.push(expr);

            // Call arguments must be postfixed by a closing parenthese or a comma
            match lexer.current_token() {
                Some(Token::Misc(')')) => break,
                Some(Token::Misc(',')) => (),
                _ => return self.log_error("Expected ')' or ','".into()),
            };
            lexer.get_next_token();
        }

        // Eat the closing parenthese
        lexer.get_next_token();

        let kind = ExprKind::Call {
            callee: identifier,
            args: call_args,
        };

        return Expr { kind }.into();
    }

    fn parse_primary_expr<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        match lexer.current_token() {
            Some(Token::Identifier(ident)) => {
                self.parse_identifier_prefixed_expr(ident.clone(), lexer)
            }
            Some(Token::Number(_)) => self.parse_number_expr(lexer).into(),
            Some(Token::Misc('(')) => self.parse_paren_expr(lexer),
            _ => self.log_error("unknown token when expecting an expression".into()),
        }
    }

    // Operator parsing and precedence stuff
    fn parse_expression<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        let primary = self.parse_primary_expr(lexer)?;
        self.parse_binary_op_rhs(0, primary, lexer)
    }

    fn parse_binary_op_rhs<L: Lex>(
        &mut self,
        lowest_edible_op_precedence: i32,
        mut lhs: Expr,
        lexer: &mut L,
    ) -> Option<Expr> {
        loop {
            // Try looking up precedence and default to -1 (which is worst than
            // any precedence) if this fails
            let (precedence, op): (i32, Option<char>) = match lexer.current_token() {
                Some(Token::Misc(c)) => (
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
            lexer.get_next_token();
            let mut rhs = self.parse_primary_expr(lexer)?;

            // Checking if there is a higher precedence operator to the RHS
            let next_precedence = match lexer.current_token() {
                Some(Token::Misc(c)) => self.environment.get_operator_precedence(*c).unwrap_or(-1),
                _ => -1,
            };
            if next_precedence > precedence {
                // If so, recurse to the rhs
                rhs = self.parse_binary_op_rhs(precedence + 1, rhs, lexer)?;
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

    fn parse_function_prototype<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        let func_name: Option<String> = match lexer.current_token() {
            Some(Token::Identifier(i)) => Some(i.clone()),
            _ => None,
        };

        // May want to consume the token here?
        if func_name.is_none() {
            return self.log_error(format!(
                "Expected function name in protype,\n  got {:#?}",
                lexer.current_token()
            ));
        }

        let func_name = func_name.unwrap();
        lexer.get_next_token();

        // Opening (
        match lexer.current_token() {
            Some(Token::Misc('(')) => (),
            _ => {
                return self.log_error(format!(
                    "Expected '(' in prototype,\n  got {:#?}",
                    lexer.current_token()
                ))
            }
        }
        lexer.get_next_token();

        let mut arg_names: Vec<String> = vec![];
        while let Some(ident) = match lexer.current_token() {
            Some(Token::Identifier(ident)) => Some(ident),
            _ => None,
        } {
            arg_names.push(ident.to_string());
            // This should be a ','
            lexer.get_next_token();

            match lexer.current_token() {
                // Reached the end of the arguments, keep this in the lexer's
                // buffer for the match following this loop
                Some(Token::Misc(')')) => (),
                // Another argument may follow (we allow trailing commas)
                Some(Token::Misc(',')) => lexer.get_next_token(),
                _ => {
                    return self.log_error(format!(
                        "Expected ',' or ')' in prototype,\n  got {:#?}",
                        lexer.current_token()
                    ))
                }
            }
        }

        match lexer.current_token() {
            Some(Token::Misc(')')) => (),
            _ => {
                return self.log_error(format!(
                    "Expected ')' in prototype,\n  got {:#?}",
                    lexer.current_token()
                ))
            }
        }
        lexer.get_next_token();

        Expr {
            kind: ExprKind::Prototype {
                args: arg_names,
                name: func_name,
            },
        }
        .into()
    }

    fn parse_function_definition<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        // Eat 'def'
        lexer.get_next_token();
        let prototype = self.parse_function_prototype(lexer)?;
        let expression = self.parse_expression(lexer)?;

        Expr {
            kind: ExprKind::Function {
                prototype: Box::new(prototype),
                body: Box::new(expression),
            },
        }
        .into()
    }

    fn parse_extern<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        lexer.get_next_token();
        self.parse_function_prototype(lexer)
    }

    // Handle top level expressions by defining zero argument functions containing the expr
    fn parse_top_level_expression<L: Lex>(&mut self, lexer: &mut L) -> Option<Expr> {
        let expression = self.parse_expression(lexer)?;
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
}

#[cfg(test)]
mod tests;
