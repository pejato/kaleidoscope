use std::collections::HashMap;

use crate::environment::Environment;

pub mod ast;
pub mod environment;
mod test_utilities;
pub mod tokenization;

fn main() {
    let environment = build_environment();
}

fn build_environment() -> Environment {
    let mut environment = Environment::new();
    [('<', 10), ('+', 20), ('-', 30), ('*', 40)]
        .iter()
        .for_each(|p| environment.add_operator_precedence(*p));

    environment
}
