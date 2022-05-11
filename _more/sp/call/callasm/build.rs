extern crate cc;

fn main() {
    cc::Build::new()
        .file("add.c")
        .compile("my-asm-lib");
}
