use std::env;
use std::collections::HashMap;

struct Dict {
    map: HashMap<String, String>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add(&mut self, k:&str, v:&str) {
        self.map.insert(k.to_string(), v.to_string());
    }

    pub fn get(&mut self, k:&str)->Option<&String> {
        return self.map.get(k);
    }
}

fn main() {
  let mut e2c = Dict::new();
  e2c.add("a", "一隻");
  e2c.add("dog", "狗");
  e2c.add("cat", "貓");
  e2c.add("chase", "追");
  e2c.add("bite", "咬");
  let args: Vec<String> = env::args().collect();
  for e in &args[1..] {
      let o = e2c.get(&e);
      match o {
        Some(c) => print!("{} ", c),
        None    => print!("None"),
      }
  }
}
