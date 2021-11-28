use std::collections::HashMap;

use llvm_sys::core::{LLVMConstReal, LLVMDoubleType};
use llvm_sys::prelude::*;

use crate::ast::{Expr, ExprKind};

pub trait CodeGen {
    type Context;
    fn codegen(&self, context: &Self::Context) -> LLVMValueRef;
}

pub struct KaleidoscopeContext {
    named_values: HashMap<String, LLVMValueRef>,
}

impl CodeGen for Expr {
    type Context = KaleidoscopeContext;

    fn codegen(&self, context: &Self::Context) -> LLVMValueRef {
        match self.kind {
            ExprKind::Number(num) => self.codegen_number(num),
            ExprKind::Variable { ref name } => self.codegen_variable(name, &context),
            ExprKind::Binary { .. } => self.codegen_binary(),
            ExprKind::Call { .. } => self.codegen_call(),
            ExprKind::Prototype { .. } => self.codegen_prototype(),
            ExprKind::Function { .. } => self.codegen_function(),
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
    ) -> LLVMValueRef {
        todo!()
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
