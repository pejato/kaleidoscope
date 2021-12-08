use llvm_sys::{
    core::{LLVMCreateBuilder, LLVMDisposeBuilder, LLVMDisposeModule, LLVMModuleCreateWithName},
    prelude::{LLVMBuilderRef, LLVMModuleRef, LLVMValueRef},
};
use std::{collections::HashMap, ffi::CString};

pub struct KaleidoscopeContext {
    pub named_values: HashMap<String, LLVMValueRef>,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,

    _c_strings: Vec<CString>,
}

impl KaleidoscopeContext {
    pub fn new() -> Self {
        unsafe {
            let _module_name = CString::new("my module").expect("CString::new failed");
            let _module_name_raw_ptr = _module_name.as_ptr();
            Self {
                named_values: HashMap::new(),
                module: LLVMModuleCreateWithName(_module_name_raw_ptr),
                // Note: This implicitly uses the global context
                builder: LLVMCreateBuilder(),
                _c_strings: vec![_module_name]
            }
        }
    }

    pub fn make_cchar_ptr(&mut self, s: &str) -> *const i8 {
        let c_str = CString::new(s).expect("CString::new failed!");
        let ptr = c_str.as_ptr();
        self._c_strings.push(c_str);
        return ptr
    }
}

impl Drop for KaleidoscopeContext {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeModule(self.module);
        }
    }
}
