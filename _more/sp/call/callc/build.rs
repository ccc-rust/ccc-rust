// build.rs
use cc;

fn main() {
  cc::Build::new()
      .file("foo.c")
      .file("bar.c")
      .compile("foo");
}
