use crate::ast::{Expr, ExprKind};
use crate::kaleidoscope_context::KaleidoscopeContext;
use inkwell::context::{self, Context};
use inkwell::values::FloatValue;
pub trait CodeGen {
    type Context;
    fn codegen(&self, context: &mut Self::Context) -> Option<FloatValue>;
}

// TODO: How should we handle LLVMValueRef potentially containing nullptr?

impl CodeGen for Expr {
    type Context = KaleidoscopeContext;

    fn codegen(&self, context: &mut Self::Context) -> Option<FloatValue> {
        match &self.kind {
            ExprKind::Number(num) => self.codegen_number(*num).into(),
            ExprKind::Variable { ref name } => self.codegen_variable(name, &context),
            ExprKind::Binary { operator, lhs, rhs } => {
                self.codegen_binary(*operator, lhs, rhs, context).into()
            }
            ExprKind::Call { callee, args } => self.codegen_call(callee, args, &context).into(),
            ExprKind::Prototype { .. } => self.codegen_prototype().into(),
            ExprKind::Function { .. } => self.codegen_function().into(),
        }
    }
}

fn log_error(message: &str) -> Option<FloatValue> {
    todo!()
}

impl Expr {
    fn codegen_number(&self, num: f64) -> FloatValue {
        let context = context::Context::create();

        todo!()
    }

    fn codegen_variable(
        &self,
        name: &String,
        context: &<Self as CodeGen>::Context,
    ) -> Option<FloatValue> {
        todo!()
    }

    fn codegen_binary(
        &self,
        op: char,
        lhs: &Box<Expr>,
        rhs: &Box<Expr>,
        context: &mut <Self as CodeGen>::Context,
    ) -> Option<FloatValue> {
        todo!()
    }

    fn codegen_call(
        &self,
        callee: &String,
        args: &Vec<Expr>,
        context: &<Self as CodeGen>::Context,
    ) -> Option<FloatValue> {
        todo!()
    }

    fn codegen_prototype(&self) -> FloatValue {
        todo!()
    }

    fn codegen_function(&self) -> FloatValue {
        todo!()
    }
}
