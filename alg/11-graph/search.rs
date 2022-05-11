use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    neighbors: HashSet<usize>,
}

#[derive(Debug)]
struct Graph {
  nodes: Vec<Node>,
}

impl Graph {
  pub fn new(n:usize, edges:Vec<(usize,usize)>) -> Self {
      let mut g = Self {
         nodes: Vec::new(),
      };
      for _ in 0..n {
        g.nodes.push({
          Node {
            neighbors: HashSet::new()
          }
        })
      }
      for e in edges {
        g.nodes[e.0].neighbors.insert(e.1);
      }
      return g;
  }
}

fn dfs(g:&Graph, ni:usize, visited:&mut Vec<bool>) { // 深度優先搜尋
  if visited[ni] { return; }
  print!("{}=>", ni);   // 否則、印出節點
  visited[ni] = true;              //   並設定為已訪問
  for nb in g.nodes[ni].neighbors.iter() { // 取出鄰居節點
    dfs(g, *nb, visited);                   //   逐一進行訪問
  }
}

fn bfs(g:&Graph, q:&mut VecDeque<usize>, visited:&mut Vec<bool>) { // 廣度優先搜尋
  if q.len() == 0 { return; } // 如果 queue 已空，則返回。
  let ni = q.pop_front().unwrap();    // 否則、取出 queue 的第一個節點。
  if !visited[ni] {            // 如果該節點尚未拜訪過。
    visited[ni] = true;       //   標示為已拜訪
  } else {                    // 否則 (已訪問過)
    return;                   //   不繼續搜尋，直接返回。
  }
  print!("{}=>", ni);       // 印出節點
  for nb in g.nodes[ni].neighbors.iter() { 
    if !visited[*nb] {      // 假如該鄰居還沒被拜訪過
      q.push_back(*nb);     //   就放入 queue 中
    }
  }
  bfs(g, q, visited);
}

fn main() {
  let n = 7;
  let g = Graph::new(n, vec![(1,2),(1,5),(2,3),(2,4), (3,4), (3,5), (3,6), (4,5), (4,6), (5,6)]);
  let mut visited = vec![false;n];
  print!("dfs:");
  dfs(&g, 1, &mut visited);
  println!("");
  let mut visited = vec![false;n];
  let mut queue = VecDeque::new();
  queue.push_back(1); // BFS 用的 queue, 起始點為 1。
  print!("bfs:");
  bfs(&g, &mut queue, &mut visited);
  println!("");
}

