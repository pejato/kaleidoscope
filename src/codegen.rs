use core::slice::SlicePattern;
use std::collections::HashMap;

use crate::ast::{Expr, ExprKind};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, FloatValue, PointerValue};
use inkwell::FloatPredicate;

pub struct CodeGen<'a, 'ctx> {
    pub builder: &'a Builder<'ctx>,
    pub context: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
    pub named_values: HashMap<String, PointerValue<'ctx>>,
}

// TODO: How should we handle LLVMValueRef potentially containing nullptr?

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn codegen(&self, expr: &Expr) -> Option<FloatValue<'ctx>> {
        match &expr.kind {
            ExprKind::Number(num) => self.codegen_number(*num).into(),
            ExprKind::Variable { ref name } => self.codegen_variable(name),
            ExprKind::Binary { operator, lhs, rhs } => {
                self.codegen_binary(*operator, lhs, rhs).into()
            }
            ExprKind::Call { callee, args } => self.codegen_call(callee, args).into(),
            ExprKind::Prototype { .. } => self.codegen_prototype().into(),
            ExprKind::Function { .. } => self.codegen_function().into(),
        }
    }
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn codegen_number(&self, num: f64) -> FloatValue<'ctx> {
        self.context.f64_type().const_float(num)
    }

    fn codegen_variable(&self, name: &String) -> Option<FloatValue<'ctx>> {
        // Note: This wouldn't work if we had non float types..
        self.named_values
            .get(name)
            .map(|val| self.builder.build_load(*val, name))
            .map(|instr| instr.into_float_value())
    }

    fn codegen_binary(
        &self,
        op: char,
        lhs: &Box<Expr>,
        rhs: &Box<Expr>,
    ) -> Option<FloatValue<'ctx>> {
        let lhs = self.codegen(lhs)?;
        let rhs = self.codegen(rhs)?;
        // inkwell::values::FloatMathValue
        match op {
            '+' => self.builder.build_float_add(lhs, rhs, "addtmp").into(),
            '-' => self.builder.build_float_sub(lhs, rhs, "subtmp").into(),
            '*' => self.builder.build_float_mul(lhs, rhs, "multmp").into(),
            '<' => {
                let cmp_as_intval =
                    self.builder
                        .build_float_compare(FloatPredicate::ULT, lhs, rhs, "cmptmp");

                self.builder
                    .build_unsigned_int_to_float(cmp_as_intval, self.context.f64_type(), "booltmp")
                    .into()
            }
            _ => {
                eprintln!("Unexpected operator {}", op);
                None
            }
        }
    }

    fn codegen_call(&self, callee: &String, args: &Vec<Expr>) -> Option<FloatValue<'ctx>> {
        let callee_fn = self.module.get_function(callee)?;

        let callee_params = callee_fn.get_params();
        if callee_params.len() != args.len() {
            return None;
        }

        let mut compiled_args = Vec::with_capacity(args.len());

        for arg in args {
            compiled_args.push(self.codegen(arg)?);
        }

        let compiled_args: Vec<BasicMetadataValueEnum> =
            compiled_args.into_iter().map(|val| val.into()).collect();

        self.builder
            .build_call(callee_fn, compiled_args.as_slice(), callee)
            .try_as_basic_value()
            .left()
            .map(|val| val.into_float_value())
    }

    fn codegen_prototype(&self) -> FloatValue<'ctx> {
        todo!()
    }

    fn codegen_function(&self) -> FloatValue<'ctx> {
        todo!()
    }
}
