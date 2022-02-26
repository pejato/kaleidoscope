use super::*;
use crate::test_utilities::test::approx_equal;
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
    eprintln!("{:#?}", result);

    assert!(approx_equal(result.get_constant().unwrap().0, 32.0, 5));
}
