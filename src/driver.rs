use inkwell::{context::Context, values::AnyValue};

use crate::{
    ast::{Expr, ExprKind},
    codegen::CodeGen,
    lexer::{Lex, Lexer, Token},
    parser::{Parse, Parser},
};

use std::io::{Read, Write};

pub trait Drive<'ctx> {
    fn new(input: Box<dyn Read>, output: Box<dyn Write>, context: &'ctx Context) -> Self;
    fn run(&mut self) -> Result<(), std::io::Error>;
    fn handle_function_definition(&mut self) -> Result<(), std::io::Error>;
    fn handle_extern(&mut self) -> Result<(), std::io::Error>;
    fn handle_top_level_expression(&mut self) -> Result<(), std::io::Error>;
}

pub struct Driver<'a> {
    parser: Parser,
    lexer: Lexer<Box<dyn Read>>,
    codegen: CodeGen<'a>,
    output: Box<dyn Write>,
}

impl<'ctx, 'a> Drive<'ctx> for Driver<'a>
where
    'ctx: 'a,
{
    fn new(input: Box<dyn Read>, output: Box<dyn Write>, context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("Kaleidoscope");
        Driver {
            parser: Parser::new(),
            lexer: Lexer::new(input),
            codegen: CodeGen::new(context, builder, module),
            output,
        }
    }
    fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            write!(self.output, "ready> ")?;
            self.output.flush()?;
            self.lexer.get_next_token();

            match self.lexer.current_token() {
                Some(Token::EOF) | None => return Ok(()),
                Some(Token::Misc(';')) => self.lexer.get_next_token(),
                Some(Token::Def) => self.handle_function_definition()?,
                Some(Token::Extern) => self.handle_extern()?,
                _ => self.handle_top_level_expression()?,
            }

            match self.lexer.current_token() {
                Some(Token::Misc(c)) => {
                    if *c != ';' {
                        writeln!(self.output, "Expected ';', but got {}", *c)?;
                    }
                }
                Some(tok) => writeln!(self.output, "Expected ';', but got {:#?}", tok)?,
                None => writeln!(self.output, "Expected ';', but got nothing...")?,
            }
        }
    }
    fn handle_function_definition(&mut self) -> Result<(), std::io::Error> {
        match self.parser.parse_function_definition(&mut self.lexer) {
            Some(expr) => {
                writeln!(self.output, "Parsed a function definition")?;
                self.output.flush()?;
                self.handle_function_codegen(&expr)?;
            }
            None => {
                writeln!(
                    self.output,
                    "Failed to parse function definition, continuing..."
                )?;
                self.output.flush()?;
                self.lexer.get_next_token();
            }
        }
        Ok(())
    }

    fn handle_extern(&mut self) -> Result<(), std::io::Error> {
        match self.parser.parse_extern(&mut self.lexer) {
            Some(expr) => {
                writeln!(self.output, "Parsed an extern")?;
                self.output.flush()?;
                self.handle_prototype_codegen(&expr)?;
            }
            None => {
                writeln!(self.output, "Failed to parse extern, continuing...")?;
                self.output.flush()?;
                self.lexer.get_next_token();
            }
        }
        Ok(())
    }

    fn handle_top_level_expression(&mut self) -> Result<(), std::io::Error> {
        match self.parser.parse_top_level_expression(&mut self.lexer) {
            Some(expr) => {
                writeln!(self.output, "Parsed a top level expression")?;
                self.output.flush()?;
                self.handle_function_codegen(&expr)?;
            }
            None => {
                writeln!(
                    self.output,
                    "Failed to parse top level definition, continuing..."
                )?;
                self.output.flush()?;
                self.lexer.get_next_token();
            }
        }
        Ok(())
    }
}

impl Driver<'_> {
    fn handle_function_codegen(&mut self, expr: &Expr) -> Result<(), std::io::Error> {
        match &expr.kind {
            ExprKind::Function { prototype, body } => {
                let result = self
                    .codegen
                    .codegen_function(prototype, body)
                    .map_or("Failed to codegen function, continuing...".into(), |ir| {
                        ir.print_to_string().to_string()
                    });
                writeln!(self.output, "{}", result)?;
                self.output.flush()?;
            }
            _ => {
                writeln!(self.output, "Failed to codegen function, continuing...")?;
                self.output.flush()?;
            }
        }
        Ok(())
    }

    fn handle_prototype_codegen(&mut self, expr: &Expr) -> Result<(), std::io::Error> {
        match &expr.kind {
            ExprKind::Prototype { name, args } => {
                let result = self
                    .codegen
                    .codegen_prototype(args, name)
                    .map_or("Failed to codegen extern, continuing...".into(), |ir| {
                        ir.print_to_string().to_string()
                    });
                writeln!(self.output, "{}", result)?;
                self.output.flush()?;
            }
            _ => {
                writeln!(self.output, "Failed to codegen extern, continuing...")?;
                self.output.flush()?;
                self.lexer.get_next_token();
            }
        }
        Ok(())
    }

    pub fn dump_ir(&mut self) -> Result<(), std::io::Error> {
        let llvm_string = self.codegen.module.print_to_string();
        let as_str = llvm_string
            .to_str()
            .ok()
            .map_or("Failed to dump Module IR", |s| s);
        writeln!(self.output, "{}", as_str)?;
        Ok(())
    }
}
