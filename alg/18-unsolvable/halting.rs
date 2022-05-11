fn halt(code:&str, input:&str)->bool {
    // 这里的H函数有两种返回值，會停(true) 或 不會停(false)
    // ... 不管你怎麼實作，假如你真的做出來了
    return code.len() > input.len(); // 這個亂寫的，你能做對嗎？
}

fn main() {
    let code = "f(s)";
    let input = "0";
    println!("halt({}, {})={}", code, input, halt(code, input));
}

/*

function f1(n) {
  return n * n
}

function f2(n) {
  let s = 0
  for (let i=0; i<n; i++) {
    for (let j=0; j<n; j++) {
      for (let k=0; k<n; k++) {
        for (let g=0; g<n; g++) {
          s = s+1
        }
      }
    }
  }
}
*/