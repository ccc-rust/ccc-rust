use std::env;
use std::collections::HashMap;

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
  let args: Vec<String> = env::args().collect();
  for e in &args[1..] {
      let o = e2c.get(e);
      match o {
        Some(c) => print!("{} ", c),
        None    => print!("None"),
      }
  }
}
