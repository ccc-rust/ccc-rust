fn is_theorem(s:&str)->bool {
  return s.len() > 10; // 這個亂寫的，你能做對嗎？
}

// 哥德爾不完備定理：不存在一個程式，可以正確判斷一個「包含算術的一階邏輯字串」是否為定理。
fn provable(s:&str)->bool {
  if is_theorem(s) { 
    return true;
  } else {
    return false;
  }
}

fn main() {
  let rule = "is_theorem('∃s -provable(s) & -provable(-s)')";
  println!("{}={}", rule, provable(rule));
}

