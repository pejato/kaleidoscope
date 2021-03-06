use inkwell::{context::Context, values::AnyValue, OptimizationLevel};
use llvm_sys::support::LLVMAddSymbol;
use scopeguard::defer;

use crate::{
    ast::{Expr, ExprKind},
    codegen::CodeGen,
    lexer::{Lex, Lexer, Token},
    option_ext::OptionExt,
    parser::{Parse, Parser},
};

use std::{
    ffi::c_void,
    io::{Read, Write},
};

pub trait Drive<'ctx> {
    fn new(input: Box<dyn Read>, output: Box<dyn Write>, context: &'ctx Context) -> Self;
    fn run(&mut self) -> Result<(), std::io::Error>;
    fn handle_function_definition(&mut self) -> Result<(), std::io::Error>;
    fn handle_extern(&mut self) -> Result<(), std::io::Error>;
    fn handle_top_level_expression(&mut self) -> Result<(), std::io::Error>;
    fn with_options(self, options: DriverOptions) -> Self;
}

#[derive(clap::Parser)]
pub struct DriverOptions {
    #[clap(long)]
    pub(crate) print_parse: bool,
    #[clap(long)]
    pub(crate) print_ir: bool,
}

pub struct Driver<'a> {
    parser: Parser,
    lexer: Lexer<Box<dyn Read>>,
    codegen: CodeGen<'a>,
    output: Box<dyn Write>,
    options: DriverOptions,
}

impl Driver<'_> {
    fn shim_lib_functions() {
        for func in &crate::library::PRINT_FNS {
            let fn_name = func.name.as_ptr();
            let fn_ptr = func.func_pointer as *mut c_void;
            unsafe { LLVMAddSymbol(fn_name, fn_ptr) };
        }
    }
}

impl<'ctx, 'a> Drive<'ctx> for Driver<'a>
where
    'ctx: 'a,
{
    fn with_options(self, options: DriverOptions) -> Self {
        Self {
            parser: self.parser,
            lexer: self.lexer,
            codegen: self.codegen,
            output: self.output,
            options,
        }
    }

    fn new(input: Box<dyn Read>, output: Box<dyn Write>, context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = context.create_module("Kaleidoscope");
        Driver {
            parser: Parser::new(),
            lexer: Lexer::new(input),
            options: DriverOptions {
                print_parse: false,
                print_ir: false,
            },
            codegen: CodeGen::new(context, builder, module),
            output,
        }
    }
    fn run(&mut self) -> Result<(), std::io::Error> {
        Self::shim_lib_functions();

        loop {
            write!(self.output, "ready> ")?;
            self.output.flush()?;
            self.lexer.get_next_token();

            match self.lexer.current_token() {
                Some(Token::EOF) | None => return Ok(()),
                Some(Token::Misc(';')) => self.lexer.get_next_token().discard(),
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
                if self.options.print_parse {
                    writeln!(self.output, "Parsed a function definition")?;
                    writeln!(self.output, "{:#?}", expr)?;
                    self.output.flush()?;
                }
                Ok(self.handle_function_codegen(&expr, false)?)
            }
            None => {
                writeln!(
                    self.output,
                    "Failed to parse function definition, continuing..."
                )?;
                self.output.flush()?;
                self.lexer.get_next_token().discard();
                Ok(())
            }
        }
    }

    fn handle_extern(&mut self) -> Result<(), std::io::Error> {
        match self.parser.parse_extern(&mut self.lexer) {
            Some(expr) => {
                if self.options.print_parse {
                    writeln!(self.output, "Parsed an extern")?;
                    writeln!(self.output, "{:#?}", expr)?;
                    self.output.flush()?;
                }
                Ok(self.handle_prototype_codegen(&expr)?)
            }
            None => {
                writeln!(self.output, "Failed to parse extern, continuing...")?;
                self.output.flush()?;
                self.lexer.get_next_token().discard();
                Ok(())
            }
        }
    }

    fn handle_top_level_expression(&mut self) -> Result<(), std::io::Error> {
        match self.parser.parse_top_level_expression(&mut self.lexer) {
            Some(expr) => {
                if self.options.print_parse {
                    writeln!(self.output, "Parsed a top level expression")?;
                    writeln!(self.output, "{:#?}", expr)?;
                    self.output.flush()?;
                }
                Ok(self.handle_function_codegen(&expr, true)?)
            }
            None => {
                writeln!(
                    self.output,
                    "Failed to parse top level definition, continuing..."
                )?;
                self.output.flush()?;
                self.lexer.get_next_token().discard();
                Ok(())
            }
        }
    }
}

impl Driver<'_> {
    fn handle_function_codegen(
        &mut self,
        expr: &Expr,
        is_anonymous: bool,
    ) -> Result<(), std::io::Error> {
        match &expr.kind {
            ExprKind::Function { prototype, body } => {
                let result = self.codegen.codegen_function(prototype, body);

                if self.options.print_ir {
                    let result_as_str = result
                        .map_or("Failed to codegen function, continuing...".into(), |ir| {
                            ir.print_to_string().to_string()
                        });
                    writeln!(self.output, "{}", result_as_str)?;
                    self.output.flush()?;
                }

                if !is_anonymous || result.is_none() {
                    return Ok(());
                }
                let result = result.unwrap();

                let engine = self
                    .codegen
                    .module
                    .create_jit_execution_engine(OptimizationLevel::Aggressive)
                    .unwrap();

                defer!(
                    engine.remove_module(&self.codegen.module).unwrap();
                );

                let fun = unsafe {
                    engine.get_function::<unsafe extern "C" fn() -> f64>(
                        result.get_name().to_str().unwrap(),
                    )
                };

                if fun.is_err() {
                    return Ok(());
                }

                let fun = fun.unwrap();
                let result = unsafe { fun.call() };
                eprintln!("Evaluated to {}\n", result);
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
                let result = self.codegen.codegen_prototype(args, name);
                if self.options.print_ir {
                    let result = result.print_to_string().to_string();
                    writeln!(self.output, "{}", result)?;
                    self.output.flush()?;
                }
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
        if !self.options.print_ir {
            return Ok(());
        }
        let llvm_string = self.codegen.module.print_to_string();
        let as_str = llvm_string
            .to_str()
            .ok()
            .map_or("Failed to dump Module IR", |s| s);
        writeln!(self.output, "{}", as_str)?;
        Ok(())
    }
}
