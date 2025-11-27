use std::collections::HashMap;

/// This is a temporary workaround, meant as a stopgap for parsing and visualizing the graph structure quickly,
/// allowing iterative development before proper algorithmic support is implemented.
///
/// ```ignore
/// import networkx as nx
///
/// graph = nx.Graph()
/// with open("test.txt") as file:
///     for line in file:
///         left, right = line.split(":")
///
///         for node in right.strip().split(" "):
///             graph.add_edge(left, node)
///             graph.add_edge(node, left)
///
/// print(graph)
/// graph.remove_edges_from(nx.minimum_edge_cut(graph))
/// a, b = nx.connected_components(graph)
/// print(len(a) * len(b))
/// ```
pub fn foo() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .for_each(|line| {
            let (key, vals) = line
                .split_once(": ")
                .expect("Line must contain a valid delimiter.");
            vals.split_whitespace().for_each(|val| {
                graph
                    .entry(key.to_string())
                    .or_insert_with(Vec::new)
                    .push(val.to_string());

                graph
                    .entry(val.to_string())
                    .or_insert_with(Vec::new)
                    .push(key.to_string());
            });
        });

    for (k, v) in &graph {
        println!("{k} -> {v:?}");
    }
}
