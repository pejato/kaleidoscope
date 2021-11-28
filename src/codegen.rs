use llvm_sys::core::{LLVMConstReal, LLVMDoubleType};
use llvm_sys::prelude::*;

use crate::ast::{Expr, ExprKind};

pub trait CodeGen {
    fn codegen(&self) -> LLVMValueRef;
}

impl CodeGen for Expr {
    fn codegen(&self) -> LLVMValueRef {
        match self.kind {
            ExprKind::Number(num) => self.codegen_number(num),
            ExprKind::Variable { .. } => self.codegen_variable(),
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

    fn codegen_variable(&self) -> LLVMValueRef {
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
