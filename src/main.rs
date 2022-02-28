use driver::{Drive, Driver};
use inkwell::context::Context;
use std::io::{stdin, stdout};

pub mod ast;
pub mod codegen;
pub mod driver;
pub mod environment;
pub mod lexer;
pub mod parser;
mod test_utilities;

fn main() -> Result<(), std::io::Error> {
    let context = Context::create();
    let mut driver = Driver::new(Box::new(stdin()), Box::new(stdout()), &context);
    driver.run()?;
    driver.dump_ir()?;
    Ok(())
}
