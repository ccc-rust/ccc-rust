# smap -- Simple-Map

```rs
use smap::Dict;

fn main() {
    let mut e2c = Dict::new();
    e2c.add("a", "一隻");
    e2c.add("dog", "狗");
    e2c.add("cat", "貓");
    e2c.add("chase", "追");
    e2c.add("bite", "咬");
    println!("{:?}", e2c.get("cat"));
    println!("{:?}", e2c.get("xxx"));
    assert!(e2c.get("cat") != None);
    assert!(e2c.get("xxx") == None);
}
```

run:

```
$ cargo run
   Compiling smap1 v0.1.0 (D:\ccc\ccc109a\se\rust\se\use\smap1)
    Finished dev [unoptimized + debuginfo] target(s) in 2.91s
     Running `target\debug\smap1.exe`
Some("貓")
None
```

