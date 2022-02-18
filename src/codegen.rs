use crate::ast::{Expr, ExprKind};
use inkwell::context::Context;
use inkwell::values::FloatValue;
pub trait CodeGen<'a, 'ctx: 'a> {
    fn codegen(&'a self, context: &'ctx mut Context) -> Option<FloatValue>;
}

// TODO: How should we handle LLVMValueRef potentially containing nullptr?

impl<'a, 'ctx: 'a> CodeGen<'a, 'ctx> for Expr {
    fn codegen(&'a self, context: &'ctx mut Context) -> Option<FloatValue<'ctx>> {
        match &self.kind {
            ExprKind::Number(num) => self.codegen_number(*num, context).into(),
            ExprKind::Variable { ref name } => self.codegen_variable(name, context),
            ExprKind::Binary { operator, lhs, rhs } => {
                self.codegen_binary(*operator, lhs, rhs, context).into()
            }
            ExprKind::Call { callee, args } => self.codegen_call(callee, args, context).into(),
            ExprKind::Prototype { .. } => self.codegen_prototype().into(),
            ExprKind::Function { .. } => self.codegen_function().into(),
        }
    }
}

impl<'a, 'ctx> Expr {
    fn codegen_number(&self, num: f64, context: &'ctx Context) -> FloatValue<'ctx> {
        context.f64_type().const_float(num)
    }

    fn codegen_variable(&self, name: &String, context: &mut Context) -> Option<FloatValue<'ctx>> {
        todo!()
    }

    fn codegen_binary(
        &self,
        op: char,
        lhs: &Box<Expr>,
        rhs: &Box<Expr>,
        context: &mut Context,
    ) -> Option<FloatValue<'a>> {
        todo!()
    }

    fn codegen_call(
        &self,
        callee: &String,
        args: &Vec<Expr>,
        context: &mut Context,
    ) -> Option<FloatValue<'ctx>> {
        todo!()
    }

    fn codegen_prototype(&self) -> FloatValue<'a> {
        todo!()
    }

    fn codegen_function(&self) -> FloatValue<'a> {
        todo!()
    }
}
