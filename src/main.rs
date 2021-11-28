use driver::{Drive, Driver};
use std::io::{stdin, stdout};

pub mod ast;
pub mod codegen;
pub mod driver;
pub mod environment;
pub mod kaleidoscope_context;
pub mod lexer;
pub mod parser;
mod test_utilities;

fn main() -> Result<(), std::io::Error> {
    let mut driver = Driver::new(Box::new(stdin()), Box::new(stdout()));
    driver.run()?;

    Ok(())
}
