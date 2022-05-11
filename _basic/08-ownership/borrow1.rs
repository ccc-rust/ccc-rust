// https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html

use std::collections::HashMap;

// 我们将获取引用作为函数参数称为 借用（borrowing）。
// 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。
pub fn add(map:&mut HashMap<String,String> , k:&str, v:&str) {
  map.insert(k.to_string(), v.to_string());
}

fn main() {
  let mut e2c = HashMap::new();
  add(&mut e2c, "a", "一隻");
  add(&mut e2c, "dog", "狗");
  add(&mut e2c, "cat", "貓");
  add(&mut e2c, "chase", "追");
  add(&mut e2c, "bite", "咬");
  println!("e2c.get(dog)={:?}", e2c.get("dog"));
  println!("e2c.get(xxx)={:?}", e2c.get("xxx"));
}
