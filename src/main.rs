use std::collections::HashMap;

use crate::parser::Parser;

pub mod ast;
pub mod environment;
pub mod parser;
mod test_utilities;
pub mod tokenization;

fn main() {
    let parser = Parser::new();
}
