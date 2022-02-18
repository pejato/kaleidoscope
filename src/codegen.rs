use scopeguard::defer;
use std::ffi::CString;
use std::ptr::null_mut;

use llvm_sys::core::{
    LLVMArrayType, LLVMBuildCall, LLVMBuildFAdd, LLVMBuildFCmp, LLVMBuildFMul, LLVMBuildFSub,
    LLVMBuildUIToFP, LLVMConstReal, LLVMCreateBuilder, LLVMDisposeBuilder, LLVMDoubleType,
    LLVMGetNamedFunction, LLVMGetNumOperands,
};
use llvm_sys::{prelude::*, LLVMRealPredicate};

use crate::ast::{Expr, ExprKind};
use crate::kaleidoscope_context::KaleidoscopeContext;

pub trait CodeGen {
    type Context;
    fn codegen(&self, context: &mut Self::Context) -> Option<LLVMValueRef>;
}

// TODO: How should we handle LLVMValueRef potentially containing nullptr?

impl CodeGen for Expr {
    type Context = KaleidoscopeContext;

    fn codegen(&self, context: &mut Self::Context) -> Option<LLVMValueRef> {
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

fn log_error(message: &str) -> Option<LLVMValueRef> {
    eprintln!("{}", message);
    None
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

    fn codegen_binary(
        &self,
        op: char,
        lhs: &Box<Expr>,
        rhs: &Box<Expr>,
        context: &mut <Self as CodeGen>::Context,
    ) -> Option<LLVMValueRef> {
        let lhs_gen = lhs.codegen(context);
        let rhs_gen = rhs.codegen(context);

        if lhs_gen.is_none() || rhs_gen.is_none() {
            return None;
        }

        let lhs_gen = lhs_gen.unwrap();
        let rhs_gen = rhs_gen.unwrap();

        unsafe {
            match op {
                '+' => LLVMBuildFAdd(
                    context.builder,
                    lhs_gen,
                    rhs_gen,
                    context.get_cchar_ptr(&"addtmp".into()),
                )
                .into(),
                '-' => LLVMBuildFSub(
                    context.builder,
                    lhs_gen,
                    rhs_gen,
                    context.get_cchar_ptr(&"subtmp".into()),
                )
                .into(),
                '*' => LLVMBuildFMul(
                    context.builder,
                    lhs_gen,
                    rhs_gen,
                    context.get_cchar_ptr(&"multmp".into()),
                )
                .into(),
                '<' => {
                    // L = Builder.CreateFCmpULT(L, R, "cmptmp");
                    let lhs_gen = LLVMBuildFCmp(
                        context.builder,
                        LLVMRealPredicate::LLVMRealULT,
                        lhs_gen,
                        rhs_gen,
                        context.get_cchar_ptr(&"cmptmp".into()),
                    );
                    LLVMBuildUIToFP(
                        context.builder,
                        lhs_gen,
                        LLVMDoubleType(),
                        context.get_cchar_ptr(&"booltmp".into()),
                    )
                    .into()
                }
                _ => log_error(&format!("Invalid binary operator {:#?}", op)),
            }
        }
    }

    fn codegen_call(
        &self,
        callee: &String,
        args: &Vec<Expr>,
        context: &<Self as CodeGen>::Context,
    ) -> Option<LLVMValueRef> {
        unsafe {
            let callee_as_cstr = CString::new(callee.as_bytes()).expect("CString new failed");
            let callee_fn = LLVMGetNamedFunction(context.module, callee_as_cstr.as_ptr());
            if callee_fn.is_null() {
                return log_error("Unknown function referenced");
            }

            let num_operands = LLVMGetNumOperands(callee_fn);
            if num_operands != args.len().try_into().ok()? {
                return log_error("Incorrect # arguments passed");
            }

            let builder = LLVMCreateBuilder();

            // let llvm_args: *mut LLVMValueRef = llmv_args.dat

            defer! { LLVMDisposeBuilder(builder); }
            // TODO: I think we should figure out whether we can just ask LLVM for this pointer..
            return LLVMBuildCall(
                builder,
                callee_fn,
                args.as_mut_ptr(),
                num_operands.try_into().ok()?,
                callee_as_cstr.as_ptr(),
            )
            .into();
        }
    }

    fn codegen_prototype(&self) -> LLVMValueRef {
        todo!()
    }

    fn codegen_function(&self) -> LLVMValueRef {
        todo!()
    }
}
