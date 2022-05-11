pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
  for (i, data) in arr.iter().enumerate() {
      if item == data {
          return Some(i);
      }
  }

  None
}

fn main() {
  let v1 = vec![1, 2, 4, 7, 9, 13, 14];
  println!("search([1, 2, 4, 7, 9, 13, 14]={:?}", linear_search(&4, &v1));
}
