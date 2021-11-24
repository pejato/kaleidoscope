use driver::{Drive, Driver};

pub mod ast;
pub mod driver;
pub mod environment;
pub mod lexer;
pub mod parser;
mod test_utilities;

fn main() {
    let driver = Driver::new();
    driver.run();
}
