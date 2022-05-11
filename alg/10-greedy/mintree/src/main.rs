mod mst;
use mst::{kruskal_mst};
use std::collections::HashMap;

//https://en.wikipedia.org/wiki/File:MST_kruskal_en.gif

fn main() {
    let vertices : Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let mut weighted_edges = HashMap::new();
    weighted_edges.insert(('c', 'd'), 2);
    weighted_edges.insert(('b', 'c'), 5);
    weighted_edges.insert(('a', 'e'), 1);
    weighted_edges.insert(('e', 'd'), 7);
    weighted_edges.insert(('a', 'b'), 3);
    weighted_edges.insert(('c', 'e'), 6);
    weighted_edges.insert(('b', 'e'), 4);

    let tree : Vec<(char, char)> = kruskal_mst(&vertices, &weighted_edges);
    println!("tree={:?}", tree);
}
