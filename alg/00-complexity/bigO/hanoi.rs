// https://github.com/TheAlgorithms/Rust/blob/master/src/general/hanoi.rs
pub fn hanoi(n: i32, from: i32, to: i32, via: i32, moves: &mut Vec<(i32, i32)>) {
  if n > 0 {
      hanoi(n - 1, from, via, to, moves);
      moves.push((from, to));
      hanoi(n - 1, via, to, from, moves);
  }
}

fn main() {
    let mut s: Vec<(i32, i32)> = Vec::new();
    hanoi(3, 1, 3, 2, &mut s);
    println!("hanoi(3,1,3,2)={:?}", s);
}
