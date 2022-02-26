use super::*;
use crate::test_utilities::test::approx_equal;
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

    assert!(approx_equal(result.get_constant().unwrap().0, 32.0, 5));
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
        builder: builder,
        module: module,
        named_values,
    };

    let result = generator.codegen_variable("x").unwrap();
    assert_eq!(result, pointer_value);
}
