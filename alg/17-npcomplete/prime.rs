// https://github.com/xcaptain/rust-algorithms/blob/master/algorithms/src/math/prime.rs
// https://en.wikipedia.org/wiki/List_of_prime_numbers
fn is_prime(n: usize) -> bool {
  let m = (n as f64).sqrt();
  let end = (m as usize) + 1;
  for i in 2..end {
      if n % i == 0 {
          return false;
      }
  }
  true
}

fn main() {
  let n = 999999000001; // 3799923412341;
  println!("is_prime(n)={}", is_prime(n));
}
