use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../input.txt");

pub fn foo() {
    let mut connections = HashMap::new();

    for line in INPUT.lines() {
        let (k, v) = line
            .split_once('-')
            .expect("Each line must contain a '-' separator.");

        connections.entry(k).or_insert_with(Vec::new).push(v);
        connections.entry(v).or_insert_with(Vec::new).push(k);
    }

    let computers = find_triangles(&connections)
        .into_iter()
        .filter(|c| [0, 3, 6].iter().any(|&i| c.chars().nth(i).unwrap() == 't'))
        .collect::<Vec<_>>();
    println!("{:?}", computers.len());
}

pub fn find_triangles(connections: &HashMap<&'static str, Vec<&'static str>>) -> Vec<String> {
    let mut triangles = Vec::new();
    let nodes = connections.keys().copied().collect::<Vec<_>>();

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

fn is_connected(
    connections: &HashMap<&'static str, Vec<&'static str>>,
    from: &str,
    to: &str,
) -> bool {
    connections
        .get(from)
        .map_or(false, |neighbors| neighbors.contains(&to))
}

pub fn interconnections(connections: &HashMap<&'static str, Vec<&'static str>>) {
    let mut visited = HashSet::new();
    for &k in connections.keys() {
        let groups = traverse(&connections, &mut visited, k, 0);
        println!("{:?}", groups);
    }
}

pub fn traverse(
    connections: &HashMap<&'static str, Vec<&'static str>>,
    visited: &mut HashSet<&'static str>,
    curr: &str,
    depth: u8,
) -> Vec<String> {
    if depth == 2 {
        return vec![curr.to_string()];
    }

    let mut res = vec![];
    if let Some(conn) = connections.get(curr) {
        for &c in conn {
            if visited.insert(c) {
                res.extend(traverse(connections, visited, c, depth + 1));
            }
        }
    }

    for val in &mut res {
        val.insert_str(0, &format!("{curr}-"));
    }
    return res;
}
