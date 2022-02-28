Working through the [LLVM Kaleidoscope tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html) using Rust.

This project uses [Inkwell](https://github.com/TheDan64/inkwell) to interface with LLVM. Originally, it used [llvm-sys](https://crates.io/crates/llvm-sys) to directly interact with LLVM's C FFI, but it became painful to reconcile C style lifetime management with the Rust ownership system.

Tentative roadmap / progress list:

- [x] [Lexer](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl01.html)
- [x] [Parsing and AST](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl02.html)
- [x] [Code Generation](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl03.html)
- [ ] [Optimization and JIT Compilation](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl04.html)
- [ ] [Control Flow](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl05.html)
- [ ] [User Defined Operations](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl06.html)
- [ ] [Mutable Variables -- SSA Form](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl07.html)
- [ ] [Object File Generation](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl08.html)
- [ ] [Generating Debug Info](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl09.html)
- [ ] Global Variables
- [ ] Additional Numeric Types
- [ ] Structs
- [ ] Arrays
- [ ] Heap Allocation
