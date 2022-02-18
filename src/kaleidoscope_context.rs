use llvm_sys::{
    core::{LLVMCreateBuilder, LLVMDisposeBuilder, LLVMDisposeModule, LLVMModuleCreateWithName},
    prelude::{LLVMBuilderRef, LLVMModuleRef, LLVMValueRef},
};
use std::{collections::HashMap, ffi::CString};

pub struct KaleidoscopeContext {
    pub named_values: HashMap<String, LLVMValueRef>,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,

    _c_string_map: HashMap<String, CString>,
}

impl KaleidoscopeContext {
    pub fn new() -> Self {
        unsafe {
            let _module_name = CString::new("my module").expect("CString::new failed");
            let _module_name_raw_ptr = _module_name.as_ptr();

            let mut map = HashMap::new();
            for instr in ["addtmp", "subtmp", "multmp", "cmptmp", "booltmp"] {
                Self::insert_as_cstr(&mut map, instr);
            }
            Self {
                named_values: HashMap::new(),
                module: LLVMModuleCreateWithName(_module_name_raw_ptr),
                // Note: This implicitly uses the global context
                builder: LLVMCreateBuilder(),
                _c_string_map: map,
            }
        }
    }

    fn insert_as_cstr(map: &mut HashMap<String, CString>, s: &str) {
        map.insert(s.to_owned(), CString::new(s).expect("CString::new failed"));
    }

    pub fn get_cchar_ptr(&self, s: &String) -> *const i8 {
        let ptr = self._c_string_map[s].as_ptr();
        return ptr;
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
