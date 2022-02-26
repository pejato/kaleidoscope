use super::*;
use inkwell::values::PointerMathValue;
use llvm_sys::LLVMValue;
use pretty_assertions::assert_eq;

#[test]
fn test_codegen_number() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();
    let generator = CodeGen {
        context: &context,
        builder: builder,
        module: module,
        named_values: HashMap::new(),
    };
    let result = generator.codegen_number(32.0);

    assert_eq!(result.get_constant().unwrap().0, 32.0);
}

#[test]
fn test_codegen_var() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    // This is super gross but all we want to do is verify that this method fetches the PointerValue corresponding to
    // the passed variable name.
    let llvm_value_mock = 42 as *mut LLVMValue;

    let pointer_value = inkwell::values::PointerValue::new(llvm_value_mock);
    let mut named_values = HashMap::new();
    named_values.insert("x".to_owned(), pointer_value);

    let generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values,
    };

    let result = generator.codegen_variable("x").unwrap();
    assert_eq!(result, pointer_value);
}

#[test]
fn test_codegen_bin_plus() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    let mut generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values: HashMap::new(),
    };

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('+', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 55.0);
}

#[test]
fn test_codegen_bin_minus() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    let mut generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values: HashMap::new(),
    };

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('-', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, -27.0);
}

#[test]
fn test_codegen_bin_mult() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    let mut generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values: HashMap::new(),
    };

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('*', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 574.0);
}

#[test]
fn test_codegen_bin_less_than_true() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    let mut generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values: HashMap::new(),
    };

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('<', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 1.0);
}

#[test]
fn test_codegen_bin_less_than_false() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    let mut generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values: HashMap::new(),
    };

    let lhs = Expr {
        kind: ExprKind::Number(41.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('<', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 0.0);
}

#[test]
fn test_codegen_bin_unknown() {
    let context = Context::create();
    let module = context.create_module("Test");
    let builder = context.create_builder();

    let mut generator = CodeGen {
        context: &context,
        builder,
        module,
        named_values: HashMap::new(),
    };

    let lhs = Expr {
        kind: ExprKind::Number(41.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('#', &lhs, &rhs);
    assert_eq!(result, None);
}
