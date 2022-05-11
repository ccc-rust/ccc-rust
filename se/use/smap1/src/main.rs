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
