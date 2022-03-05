use std::cell::RefCell;
use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{
    AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue,
    FunctionValue,
};
use inkwell::FloatPredicate;
use inkwell::OptimizationLevel::Aggressive;

use crate::ast::Expr;
use crate::ast::ExprKind;
use crate::ast::IfVal;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: RefCell<Module<'ctx>>,
    pub current_function: Option<FunctionValue<'ctx>>,
    pub function_pass_manager: PassManager<FunctionValue<'ctx>>,
    pub named_values: HashMap<String, AnyValueEnum<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, builder: Builder<'ctx>, module: Module<'ctx>) -> Self {
        let function_pass_manager_builder = PassManagerBuilder::create();
        function_pass_manager_builder.set_optimization_level(Aggressive);

        let function_pass_manager = PassManager::create(&module);
        // TODO: Look through the passes available to us.. there are a lot!
        function_pass_manager_builder.populate_function_pass_manager(&function_pass_manager);
        function_pass_manager.add_aggressive_inst_combiner_pass();
        function_pass_manager.add_reassociate_pass();
        function_pass_manager.add_new_gvn_pass();
        function_pass_manager.add_cfg_simplification_pass();

        function_pass_manager.initialize();

        CodeGen {
            builder,
            context,
            module: RefCell::new(module),
            function_pass_manager: function_pass_manager,
            named_values: HashMap::new(),
            current_function: None,
        }
    }

    pub fn codegen(&mut self, expr: &Expr) -> Option<AnyValueEnum<'ctx>> {
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
                .as_any_value_enum()
                .into(),

            ExprKind::Function { prototype, body } => self
                .codegen_function(prototype, body)
                .map(|val| val.as_any_value_enum()),

            ExprKind::If(if_payload) => self.codegen_if(if_payload),
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn codegen_number(&self, num: f64) -> FloatValue<'ctx> {
        self.context.f64_type().const_float(num)
    }

    pub fn codegen_variable(&self, name: &str) -> Option<AnyValueEnum<'ctx>> {
        self.named_values.get(name).cloned()
    }

    pub fn codegen_binary(&mut self, op: char, lhs: &Expr, rhs: &Expr) -> Option<FloatValue<'ctx>> {
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

    pub fn codegen_call(&mut self, callee: &str, args: &[Expr]) -> Option<FloatValue<'ctx>> {
        let callee_fn = self.module.borrow().get_function(callee)?;

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
            .build_call(callee_fn, compiled_args.as_slice(), "call_tmp")
            .try_as_basic_value()
            .left()
            .map(|val| val.into_float_value())
    }

    pub fn codegen_prototype(&self, args: &[String], name: &str) -> FunctionValue<'ctx> {
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
            .borrow()
            .add_function(name, fn_type, Linkage::External.into());

        for (param, arg) in the_fn.get_param_iter().zip(args.iter()) {
            param.set_name(arg);
        }

        the_fn
    }

    pub fn codegen_function(
        &mut self,
        prototype: &Expr,
        body: &Expr,
    ) -> Option<FunctionValue<'ctx>> {
        let (fn_name, args) = match &prototype.kind {
            ExprKind::Prototype { name, args } => Some((name, args)),
            _ => None,
        }?;

        // Not the cleanest, perse. It would be better to add a tag to the function prototype
        if self.module.borrow().get_function(&fn_name).is_some() && fn_name != "__anon" {
            eprintln!("Unable to redefine func {}", fn_name);
            return None;
        }
        let the_fn = self.codegen_prototype(args, fn_name);
        let bb = self.context.append_basic_block(the_fn, "entry");

        self.builder.position_at_end(bb);

        self.named_values.clear();

        for param in the_fn.get_param_iter() {
            let param_as_float: FloatValue = if param.is_float_value() {
                Some(param.into_float_value())
            } else {
                None
            }?;

            let param_name = param_as_float
                .get_name()
                .to_str()
                .ok()
                .map(|s| s.to_string())?;

            self.named_values
                .insert(param_name, param.as_any_value_enum());
        }

        self.current_function = Some(the_fn);

        match self.codegen(body) {
            Some(value) => {
                if value.is_function_value() || value.is_array_value() {
                    self.current_function = None;
                    return None;
                }
                let value: BasicValueEnum = value.try_into().ok()?;
                self.builder.build_return(Some(&value));

                if the_fn.verify(true) {
                    self.function_pass_manager.run_on(&the_fn);
                    Some(the_fn)
                } else {
                    self.current_function = None;
                    unsafe { the_fn.delete() };
                    None
                }
            }
            None => {
                // We may have created a function while recursing inside codegen and need to clear it, if so.
                self.current_function = None;
                unsafe { the_fn.delete() };
                None
            }
        }
    }

    pub fn codegen_if(&mut self, if_val: &IfVal) -> Option<AnyValueEnum<'ctx>> {
        let cond_ir = self.codegen(&if_val.if_boolish_test)?;

        if !cond_ir.get_type().is_float_type() {
            return None;
        }
        let cond_ir = cond_ir.into_float_value();

        let current_function = &self.current_function?;

        let then_block = self.context.append_basic_block(*current_function, "then");
        let else_block = self.context.append_basic_block(*current_function, "else");
        let continuation_block = self.context.append_basic_block(*current_function, "cont");

        // i1 that is true if not equal to zero, and false if it is
        let comparison = self.builder.build_float_compare(
            FloatPredicate::ONE,
            cond_ir,
            self.context.f64_type().const_float_from_string("0.0"),
            "comp",
        );

        // Conditionally branch to then and else
        self.builder
            .build_conditional_branch(comparison, then_block, else_block);

        // Codegen `then` and br to continuation block
        self.builder.position_at_end(then_block);

        let then_ir = self.codegen(&if_val.then)?;
        if !then_ir.get_type().is_float_type() {
            return None;
        }

        self.builder.position_at_end(then_block);
        self.builder.build_unconditional_branch(continuation_block);
        let then_block = self.builder.get_insert_block()?;

        // Codegen `else` br to continuation block
        self.builder.position_at_end(else_block);
        let else_ir = self.codegen(&if_val.elves)?;
        if !else_ir.get_type().is_float_type() {
            return None;
        }
        self.builder.position_at_end(else_block);
        self.builder.build_unconditional_branch(continuation_block);

        // Setting up the phi node
        self.builder.position_at_end(continuation_block);
        let phi = self.builder.build_phi(self.context.f64_type(), "iftmp");

        let then_ir: BasicValueEnum = then_ir.try_into().ok()?;
        let else_ir: BasicValueEnum = else_ir.try_into().ok()?;
        phi.add_incoming(&[(&then_ir, then_block), (&else_ir, else_block)]);

        Some(phi.as_any_value_enum())
    }
}

#[cfg(test)]
mod tests;
