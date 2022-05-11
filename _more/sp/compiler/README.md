
## Compiler

* https://crates.io/crates/llvm-sys
* https://crates.io/crates/llvm-ir
* [Introducing MIR](https://blog.rust-lang.org/2016/04/19/MIR.html)
* [Any way to get rustc to emit fully linked LLVM IR?](https://www.reddit.com/r/rust/comments/9rnmfs/any_way_to_get_rustc_to_emit_fully_linked_llvm_ir/)
    * You could also try compiling with `-Clto`. That should pull all LLVM IR from upstream crates (include `libcore` and `libstd`) into the final module emitted by `rustc`.

## Parser

* https://lib.rs/parsing
* https://lib.rs/parser-implementations
