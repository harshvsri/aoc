use std::collections::{HashMap, HashSet};

pub fn solve() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let mut connections = HashMap::new();
    content.lines().for_each(|line| {
        let (k, v) = line
            .split_once('-')
            .expect("Each line must contain a '-' separator.");

        connections.entry(k).or_insert(Vec::new()).push(v);
        connections.entry(v).or_insert(Vec::new()).push(k);
    });

    let tripods = find_tripods(&connections)
        .iter()
        .filter(|c| [0, 3, 6].iter().any(|&i| c.chars().nth(i).unwrap() == 't'))
        .count();
    println!("{:?}", tripods);

    interconnections(&connections);
}

pub fn find_tripods<'a>(connections: &HashMap<&'a str, Vec<&'a str>>) -> Vec<String> {
    fn is_connected<'a>(
        connections: &HashMap<&'a str, Vec<&'a str>>,
        from: &str,
        to: &str,
    ) -> bool {
        connections
            .get(from)
            .map_or(false, |neighbors| neighbors.contains(&to))
    }

    let mut triangles = Vec::new();
    let nodes = connections.keys().collect::<Vec<_>>();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            for k in (j + 1)..nodes.len() {
                let (a, b, c) = (nodes[i], nodes[j], nodes[k]);

                if is_connected(connections, a, b)
                    && is_connected(connections, b, c)
                    && is_connected(connections, c, a)
                {
                    triangles.push(format!("{a}-{b}-{c}"));
                }
            }
        }
    }
    triangles
}

// INFO: We need to find the largest group where every node is connected to every other node.
// In graph theory, a fully connected subgraph is called a "Clique", and we are looking for the Maximum Clique.
// This makes this problem significantly different from the largest island (connected component) problem.
pub fn interconnections<'a>(connections: &HashMap<&'a str, Vec<&'a str>>) {
    let mut curr_clique = HashSet::new();
    let mut max_clique = HashSet::new();
    let mut candidates = connections.keys().copied().collect();

    find_cliques(
        &mut curr_clique,
        &mut max_clique,
        &mut candidates,
        connections,
    );

    let mut result: Vec<&str> = max_clique.into_iter().collect();
    result.sort_unstable();
    println!("{}", result.join(","));
}

pub fn find_cliques<'a>(
    curr_clique: &mut HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
    candidates: &mut HashSet<&'a str>,
    connections: &HashMap<&'a str, Vec<&'a str>>,
) {
    if candidates.is_empty() {
        if curr_clique.len() > max_clique.len() {
            *max_clique = curr_clique.clone();
        }
        return;
    }

    // Clone the candidates so we can safely iterate over them while modifying the original set.
    let candidates_clone = candidates.clone();
    for node in candidates_clone {
        curr_clique.insert(node);

        let neighbors = if let Some(n) = connections.get(node) {
            n.iter().copied().collect()
        } else {
            HashSet::new()
        };
        let mut next_candidates = candidates.intersection(&neighbors).copied().collect();
        find_cliques(curr_clique, max_clique, &mut next_candidates, connections);

        curr_clique.remove(node);
        candidates.remove(node);
    }
}
