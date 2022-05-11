//! Solves [Water Tree](http://codeforces.com/contest/343/problem/D).
//! To make a self-contained file for contest submission, dump each desired
//! module's contents directly here instead of the use statements.
//! Also, use the commented code in main() to employ standard I/O.
extern crate graph;
use graph::graph::Graph;
use std::io;

const SAMPLE_INPUT: &[u8] = b"\
5
1 2
5 1
2 3
4 2
12
1 1
2 3
3 1
3 2
3 3
3 4
1 2
2 4
3 1
3 3
3 4
3 5
";
const SAMPLE_OUTPUT: &[u8] = b"\
0
0
0
1
0
1
0
1
";

fn dfs(
    graph: &Graph,
    u: usize,
    l: &mut [usize],
    r: &mut [usize],
    p: &mut [usize],
    time: &mut usize,
) {
    *time += 1;
    l[u] = *time;

    for (_, v) in graph.adj_list(u) {
        if l[v] == 0 {
            p[v] = l[u];
            dfs(graph, v, l, r, p, time);
        }
    }

    r[u] = *time;
}

#[test]
fn main() {
    let mut scan = Scanner::new(SAMPLE_INPUT);
    let mut out = vec![];
    solve(&mut scan, &mut out);

    assert_eq!(out, SAMPLE_OUTPUT);
}
