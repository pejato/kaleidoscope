use llvm_sys::prelude::LLVMValueRef;
use std::collections::HashMap;

pub struct KaleidoscopeContext {
    pub named_values: HashMap<String, LLVMValueRef>,
}

impl KaleidoscopeContext {
    pub fn new() -> Self {
        Self {
            named_values: HashMap::new(),
        }
    }
}
