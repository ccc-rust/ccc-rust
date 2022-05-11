// https://github.com/xcaptain/rust-algorithms/blob/master/algorithms/src/backtrack/nqueen.rs

pub fn nqueen(n: usize) -> Vec<Vec<u128>> {
  let mut board = vec![0; n];
  let mut result = vec![];
  backtrack(&mut board, 0, &mut result);
  result
}

/// algorithms from: http://jeffe.cs.illinois.edu/teaching/algorithms/book/02-backtracking.pdf
/// board: contains an available placement for current board
/// r: the current placement row, starting from 0 to n-1
/// result: the available placement array
fn backtrack(board: &mut Vec<u128>, r: usize, result: &mut Vec<Vec<u128>>) {
  let n = board.len();
  if r == n {
      result.push(board.clone());
  } else {
      for j in 0..n {
          let mut legal = true;
          for (i, boardi) in board.iter().enumerate().take(r) {
              if *boardi == (j as u128)
                  || (j + r >= i && *boardi == ((j + r - i) as u128))
                  || (j + i >= r && *boardi == ((j + i - r) as u128))
              {
                  legal = false;
              }
          }
          if legal {
              board[r] = j as u128;
              backtrack(board, r + 1, result);
          }
      }
  }
}

fn main() {
    let n = 4;
    println!("nqueen({})={:?}", n, nqueen(n));
}
