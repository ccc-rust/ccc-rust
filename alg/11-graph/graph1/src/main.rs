mod graph;

fn adj_list() {
    let mut graph = Graph::new(4, 4);
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(3, 0);

    let adj: Vec<(usize, usize)> = graph.adj_list(1).collect();

    println!("adj(1)={}", adj);
}

fn main() {
    adj_list();
}
