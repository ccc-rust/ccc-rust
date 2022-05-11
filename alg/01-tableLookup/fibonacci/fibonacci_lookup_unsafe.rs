use std::time::SystemTime;

static mut FIB: Vec<usize> = Vec::new();

fn fibonacci (n:usize) -> usize {
  unsafe {
    if FIB.len() > n { return FIB[n]; }
    FIB.push(fibonacci(n - 1) + fibonacci(n - 2));
    return FIB[n];
  }
}

fn main() {
  unsafe {
    FIB.push(0);
    FIB.push(1);
  }
  let n = 38;
  let start_time = SystemTime::now();
  println!("start_time:{:?}", start_time);
  println!("fibonacci({})={}", n, fibonacci(n));
  let end_time = SystemTime::now();
  println!("end_time:{:?}", end_time);
  let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
  println!("duration:{:?}", duration);
}
