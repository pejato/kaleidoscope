use std::collections::HashMap;

pub struct Environment {
    operator_precedence: HashMap<char, u32>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            operator_precedence: HashMap::new(),
        }
    }
    pub fn get_operator_precedence(&self, operator: char) -> Option<u32> {
        return self.operator_precedence.get(&operator).copied();
    }
    pub fn add_operator_precedence(&mut self, op_precedence_pair: (char, u32)) {
        self.operator_precedence
            .insert(op_precedence_pair.0, op_precedence_pair.1);
    }
}
