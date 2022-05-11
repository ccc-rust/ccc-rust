// 參考： 自然語言處理 -- Hidden Markov Model http://cpmarkchang.logdown.com/posts/192352

use std::collections::HashMap;

fn markov(s:&Vec<&str>, p:&HashMap<String, f64>)->f64 {
    let mut sp;
    let x = p.get(s[0]);
    match x {
        Some(prob) => sp = *prob,
        None => panic!("Error!"),
    }
    let slen = s.len();
    for i in 1..slen {
      let key = format!("{}=>{}", s[i-1], s[i]);
      let o = p.get(&key);
      match o {
          Some(prob) => sp *= prob,
          None => println!("Error!"),
      }
    }
    return sp
}

/*
const p = {
  'a': 0.2, 'b': 0.8,
  'a=>a': 0.7, 'a=>b':0.3,
  'b=>a': 0.5, 'b=>b':0.5,
}
*/
  
fn main() {
    let mut p = HashMap::new();
    p.insert("a".to_string(), 0.2);
    p.insert("b".to_string(), 0.8);
    p.insert("a=>a".to_string(), 0.7);
    p.insert("a=>b".to_string(), 0.3);
    p.insert("b=>a".to_string(), 0.5);
    p.insert("b=>b".to_string(), 0.5);
    let seq = vec!["b", "a", "b", "b"];
    // println!("p({:?})={}", seq, markov(seq));
    println!("P({:?})={}", seq, markov(&seq, &p));
}
