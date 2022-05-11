use std::collections::BTreeMap;
use std::ops::Bound::{Included, Excluded};

fn graph(edges:Vec<(u32,u32)>)->BTreeMap<(u32,u32), i32> {
  let mut g = BTreeMap::new();
  for e in edges {
    g.insert(e, 0i32);
  }
  return g;
}

fn main() {
  let g = graph(vec![(1,2),(1,3),(2,3),(2,4), (3,4)]);
  println!("g={:?}", g);
  println!("g.get((2,3))={:?}", g.get(&(2,3)));
  println!("g.get((1,4))={:?}", g.get(&(1,4)));
  for (&e,&_) in g.range((Included(&(2,0)),Excluded(&(3,0)))) {
    println!("{:?} is edge", e);
  }
}
