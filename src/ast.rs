struct Expr {
    kind: ExprKind,
}

enum ExprKind {
    Number {
        value: f64,
    },
    Variable {
        name: String,
    },
    Binary {
        operator: char,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Prototype {
        name: String,
        args: Vec<String>,
    },
    Function {
        proto: Box<Expr>,
        body: Vec<Expr>,
    },
}

// Primary expression parsing
fn parse_number_expr() -> ! {
    todo!()
}

fn parse_paren_expr() -> ! {
    todo!()
}

fn parse_identifier_expr() -> ! {
    todo!()
}

fn parse_primary_expr() -> ! {
    todo!()
}

// Operator parsing and precedence stuff
fn parse_expression() -> ! {
    todo!()
}

fn parse_binary_op_rhs() -> ! {
    todo!()
}

fn parse_function_prototype() -> ! {
    todo!()
}

fn parse_function_definition() -> ! {
    todo!()
}

fn parse_extern() -> ! {
    todo!()
}

// Handle top level expressions by defining zero argument functions containing the expr
fn parse_top_level_expression() -> ! {
    todo!()
}
