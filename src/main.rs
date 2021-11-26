use driver::{Drive, Driver};

pub mod ast;
pub mod driver;
pub mod environment;
pub mod lexer;
pub mod parser;
mod test_utilities;

fn main() -> Result<(), std::io::Error> {
    let mut driver = Driver::new();
    driver.run()?;

    Ok(())
}
