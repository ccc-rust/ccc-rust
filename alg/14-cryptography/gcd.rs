// https://github.com/xcaptain/rust-algorithms/blob/master/algorithms/src/math/gcd.rs
pub fn gcd(m: usize, n: usize) -> usize {
  let mut m = m;
  let mut n = n;
  while n != 0 {
      let t = n;
      n = m % n;
      m = t;
  }
  m
}

fn main() {
  println!("gcd(39, 26)={}", gcd(39, 26));
}
