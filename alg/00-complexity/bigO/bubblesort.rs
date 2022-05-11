// https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/bubble_sort.rs
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
  for i in 0..arr.len() {
      for j in 0..arr.len() - 1 - i {
          if arr[j] > arr[j + 1] {
              arr.swap(j, j + 1);
          }
      }
  }
}

fn main() {
  let mut v1 = vec![3, 8, 2, 1, 5];
  bubble_sort(&mut v1);
  println!("sort([3, 8, 2, 1, 5]={:?}", v1);
}
