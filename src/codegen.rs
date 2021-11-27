use llvm_sys::core::LLVMConstReal;
use llvm_sys::core::LLVMConstReal;
use llvm_sys::core::LLVMContextCreate;
use llvm_sys::LLVMContext;

unsafe fn stuff() {
    let context = LLVMContextCreate();
    let val = LLVMConstReal(context, 64.0);
}
