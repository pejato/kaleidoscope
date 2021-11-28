use llvm_sys::core::{LLVMConstReal, LLVMDoubleType};
use llvm_sys::LLVMContext;

unsafe fn stuff() {
    let mut double_type = LLVMDoubleType();
    let val = LLVMConstReal(double_type, 64.0);
}
