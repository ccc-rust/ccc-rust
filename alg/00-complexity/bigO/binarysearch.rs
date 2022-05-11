pub fn binary_search<T: PartialEq + PartialOrd>(item: &T, arr: &[T]) -> Option<usize> {
  if arr.is_empty() {
      return None;
  }

  let mut left = 0;
  let mut right = arr.len() - 1;

  while left < right {
      let mid = left + (right - left) / 2;

      if &arr[mid] > item {
          right = mid - 1;
      } else if &arr[mid] < item {
          left = mid + 1;
      } else {
          left = mid;
          break;
      }
  }

  if &arr[left] == item {
      Some(left)
  } else {
      None
  }
}

fn main() {
  let v1 = vec![1, 2, 4, 7, 9, 13, 14];
  println!("search([1, 2, 4, 7, 9, 13, 14]={:?}", binary_search(&4, &v1));
}
