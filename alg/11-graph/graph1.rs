use std::collections::BTreeMap;
use std::ops::Bound::{Included, Excluded};

#[derive(Debug)]
enum State {
  Visited,
  Unvisited,
}

struct Node {
    visited: bool, 
    neighbors: HashSet<u32>,
}

struct Graph {
  nodes: Vec<Node>,
}

/*
fn graph(edges:Vec<(u32,u32)>)->Graph {
  let mut g = Graph::new();
  for e in edges {
    g.insert(e);
  }
  return g;
}
*/
/*
fn dfs(g:&mut Graph, node:u32) { // 深度優先搜尋
  match g.get(node) {
    Some(s)=> if s == State::Visited { return; }, // 如果已訪問過，就不再訪問
    None=> return,
  }
  println!("{}=>", node);       // 否則、印出節點
  g.set(node, State::Visited);  //   並設定為已訪問
  let neighbors = g.range((Included(&(node,0)),Excluded(&(node+1,0))));
  for (&e, &_) in neighbors {   // 取出鄰居節點
    dfs(g, e[1]);               //   逐一進行訪問
  }
}
*/
fn main() {
  let g = graph(vec![(1,2),(1,5),(2,3),(2,4), (3,4), (3,5), (3,6), (4,5), (4,6), (5,6)]);
  println!("g={:?}", g);
  println!("g.get((2,3))={:?}", g.get(&(2,3)));
  println!("g.get((1,4))={:?}", g.get(&(1,4)));
  for (&e,&_) in g.range((Included(&(2,0)),Excluded(&(3,0)))) {
    println!("{:?} is edge", e);
  }
}

