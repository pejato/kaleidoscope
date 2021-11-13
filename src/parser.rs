use crate::environment::Environment;

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
}
