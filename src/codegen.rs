use llvm_sys::core::{LLVMConstReal, LLVMDoubleType};
use llvm_sys::prelude::*;

use crate::ast::{Expr, ExprKind};
use crate::kaleidoscope_context::KaleidoscopeContext;

pub trait CodeGen {
    type Context;
    fn codegen(&self, context: &Self::Context) -> Option<LLVMValueRef>;
}

// TODO: How should we handle LLVMValueRef potentially containing nullptr?

impl CodeGen for Expr {
    type Context = KaleidoscopeContext;

    fn codegen(&self, context: &Self::Context) -> Option<LLVMValueRef> {
        match self.kind {
            ExprKind::Number(num) => self.codegen_number(num).into(),
            ExprKind::Variable { ref name } => self.codegen_variable(name, &context),
            ExprKind::Binary { .. } => self.codegen_binary().into(),
            ExprKind::Call { .. } => self.codegen_call().into(),
            ExprKind::Prototype { .. } => self.codegen_prototype().into(),
            ExprKind::Function { .. } => self.codegen_function().into(),
        }
    }
}

impl Expr {
    fn codegen_number(&self, num: f64) -> LLVMValueRef {
        unsafe { return LLVMConstReal(LLVMDoubleType(), num) }
    }

    fn codegen_variable(
        &self,
        name: &String,
        context: &<Self as CodeGen>::Context,
    ) -> Option<LLVMValueRef> {
        context.named_values.get(name).map(|v| *v)
    }

    fn codegen_binary(&self) -> LLVMValueRef {
        todo!()
    }

    fn codegen_call(&self) -> LLVMValueRef {
        todo!()
    }

    fn codegen_prototype(&self) -> LLVMValueRef {
        todo!()
    }

    fn codegen_function(&self) -> LLVMValueRef {
        todo!()
    }
}
