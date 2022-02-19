use std::collections::HashMap;

use crate::ast::{Expr, ExprKind};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{
    AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue,
    FunctionValue, PointerValue,
};
use inkwell::FloatPredicate;

pub struct CodeGen<'a, 'ctx> {
    pub builder: &'a Builder<'ctx>,
    pub context: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
    pub named_values: HashMap<String, PointerValue<'ctx>>,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn codegen(&mut self, expr: &Expr) -> Option<AnyValueEnum<'ctx>> {
        match &expr.kind {
            ExprKind::Number(num) => self.codegen_number(*num).as_any_value_enum().into(),

            ExprKind::Variable { ref name } => self
                .codegen_variable(name)
                .map(|val| val.as_any_value_enum()),

            ExprKind::Binary { operator, lhs, rhs } => self
                .codegen_binary(*operator, lhs, rhs)
                .map(|val| val.as_any_value_enum()),

            ExprKind::Call { callee, args } => self
                .codegen_call(callee, args)
                .map(|val| val.as_any_value_enum()),

            ExprKind::Prototype { args, name } => self
                .codegen_prototype(args, name)
                .map(|val| val.as_any_value_enum()),

            ExprKind::Function { prototype, body } => self
                .codegen_function(prototype, body)
                .map(|val| val.as_any_value_enum()),
        }
    }
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn codegen_number(&self, num: f64) -> FloatValue<'ctx> {
        self.context.f64_type().const_float(num)
    }

    fn codegen_variable(&self, name: &str) -> Option<FloatValue<'ctx>> {
        // Note: This wouldn't work if we had non float types..
        self.named_values
            .get(name)
            .map(|val| self.builder.build_load(*val, name))
            .map(|instr| instr.into_float_value())
    }

    fn codegen_binary(&mut self, op: char, lhs: &Expr, rhs: &Expr) -> Option<FloatValue<'ctx>> {
        let lhs: FloatValue = self.codegen(lhs)?.try_into().ok()?;
        let rhs: FloatValue = self.codegen(rhs)?.try_into().ok()?;

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

    fn codegen_call(&mut self, callee: &str, args: &[Expr]) -> Option<FloatValue<'ctx>> {
        let callee_fn = self.module.get_function(callee)?;

        let callee_params = callee_fn.get_params();
        if callee_params.len() != args.len() {
            return None;
        }

        let mut compiled_args: Vec<FloatValue> = Vec::with_capacity(args.len());

        for arg in args {
            compiled_args.push(self.codegen(arg)?.try_into().ok()?);
        }

        let compiled_args: Vec<BasicMetadataValueEnum> =
            compiled_args.into_iter().map(|val| val.into()).collect();

        self.builder
            .build_call(callee_fn, compiled_args.as_slice(), callee)
            .try_as_basic_value()
            .left()
            .map(|val| val.into_float_value())
    }

    fn codegen_prototype(&self, args: &[String], name: &str) -> Option<FunctionValue<'ctx>> {
        let param_types: Vec<BasicMetadataTypeEnum> = args
            .iter()
            .map(|_| self.context.f64_type().into())
            .collect();

        let fn_type = self
            .context
            .f64_type()
            .fn_type(param_types.as_slice(), false);

        let the_fn = self
            .module
            .add_function(name, fn_type, Linkage::External.into());

        // TODO: Does this work as expected?
        for (index, param) in the_fn.get_param_iter().enumerate() {
            param.set_name(&index.to_string());
        }

        Some(the_fn)
    }

    fn codegen_function(&mut self, prototype: &Expr, body: &Expr) -> Option<FunctionValue<'ctx>> {
        let (fn_name, args) = match &prototype.kind {
            ExprKind::Prototype { name, args } => Some((name, args)),
            _ => None,
        }?;

        let the_fn = self
            .module
            .get_function(fn_name)
            .or_else(|| self.codegen_prototype(args, fn_name))?;

        // Checking to see if the fn has already been defined. This is how LLVM's Function.empty() works under the hood
        if the_fn.count_basic_blocks() > 0 {
            eprintln!("Function {} cannot be redefined.", fn_name);
            return None;
        }

        let bb = self.context.append_basic_block(the_fn, "entry");
        self.builder.position_at_end(bb);

        // TODO: This is how the Kaleidoscope tutorial does it, but it feels kinda yucky to mutate state like this..?
        self.named_values.clear();
        for param in the_fn.get_param_iter() {
            self.named_values
                .insert(fn_name.clone(), param.into_pointer_value());
        }

        match self.codegen(body) {
            Some(value) => {
                let value: BasicValueEnum = value.try_into().ok()?;
                self.builder.build_return(Some(&value));

                if the_fn.verify(true) {
                    Some(the_fn)
                } else {
                    unsafe { the_fn.delete() };
                    None
                }
            }
            None => {
                unsafe { the_fn.delete() };
                None
            }
        }
    }
}
