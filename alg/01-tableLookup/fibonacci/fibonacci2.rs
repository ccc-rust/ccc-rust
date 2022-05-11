use std::time::{SystemTime};

fn fibonacci(n:i64)->i64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let start = SystemTime::now();
    let r = fibonacci(45);
    let end = SystemTime::now();
    let duration = end
        .duration_since(start)
        .expect("Time went backwards");
    println!("fibonacci(45)={}, duration={:?}", r, duration);
}
