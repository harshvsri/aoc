use std::collections::HashMap;

pub fn dfs<'a>(
    node: &'a str,
    target: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(&val) = cache.get(node) {
        return val;
    }
    if node == target {
        return 1;
    }

    let mut count = 0;
    if let Some(connections) = graph.get(node) {
        for conn in connections {
            count += dfs(conn, target, graph, cache);
        }
    }

    cache.insert(node, count);
    return count;
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let graph = data
        .lines()
        .map(|l| {
            let (k, v) = l.split_once(": ").expect("Must contain a valid delemeter.");
            let v = v.split_whitespace().collect::<Vec<_>>();
            (k, v)
        })
        .collect::<HashMap<_, _>>();

    let res = dfs("you", "out", &graph, &mut HashMap::new());
    println!("Res: {res}");
}
