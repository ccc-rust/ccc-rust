use std::time::SystemTime;

struct Fib {
  v: Vec<u64>
}

impl Fib {
  pub fn new() -> Self {
      Self {
          v: vec![0,1]
      }   
  }   

  pub fn get(&mut self, n:usize) -> u64 {
      if self.v.len() > n { return self.v[n]; }
      let fibn = self.get(n - 1) + self.get(n - 2);
      self.v.push(fibn);
      return self.v[n];
  }
}

fn main() {
  let mut fib = Fib::new();
  let n = 38;
  let start_time = SystemTime::now();
  println!("start_time:{:?}", start_time);
  println!("fibonacci({})={}", n, fib.get(n));
  let end_time = SystemTime::now();
  println!("end_time:{:?}", end_time);
  let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
  println!("duration:{:?}", duration);
}
