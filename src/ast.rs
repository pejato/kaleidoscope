#[derive(Debug, PartialEq, PartialOrd)]
pub struct Expr {
    pub kind: ExprKind,
}
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ExprKind {
    Number(f64),
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
        prototype: Box<Expr>,
        body: Box<Expr>,
    },
}
