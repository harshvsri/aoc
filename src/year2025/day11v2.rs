use crate::year2025::day11::dfs;
use std::collections::{HashMap, HashSet};

// WARN: I have a very strong feeling that the cache and the visited are conflicting.
// There is something about this usage that just doesnt feels right.

pub fn adv_dfs<'a>(
    node: &'a str,
    target: &'a str,
    flags: [bool; 2],
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, [bool; 2]), u64>,
    visited: &mut HashSet<(&'a str, [bool; 2])>,
) -> u64 {
    if let Some(&val) = cache.get(&(node, flags)) {
        return val;
    }
    if node == target && flags.iter().all(|&f| f == true) {
        return 1;
    }
    if !visited.insert((node, flags)) {
        return 0;
    }

    let mut count = 0;
    if let Some(connections) = graph.get(node) {
        for conn in connections {
            count += match *conn {
                "dac" => adv_dfs(conn, target, [true, flags[1]], graph, cache, visited),
                "fft" => adv_dfs(conn, target, [flags[0], true], graph, cache, visited),
                _ => adv_dfs(conn, target, flags, graph, cache, visited),
            };
        }
    }

    visited.remove(&(node, flags));
    cache.insert((node, flags), count);
    return count;
}

// INFO: I found this on internet that seems to work but feels cheating.
// dac_out = dfs("dac", "out", memo={})
// fft_dac = dfs("fft", "dac", memo={})
// svr_fft = dfs("svr", "fft", memo={})
// fft_out = dfs("fft", "out", memo={})
// dac_fft = dfs("dac", "fft", memo={})
// svr_dac = dfs("svr", "dac", memo={})
// print((svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out))

pub fn solve() {
    let data =
        std::fs::read_to_string("test.txt").expect("File must be present in the root directory.");

    let graph = data
        .lines()
        .map(|l| {
            let (k, v) = l.split_once(": ").expect("Must contain a valid delemeter.");
            let v = v.split_whitespace().collect::<Vec<_>>();
            (k, v)
        })
        .collect::<HashMap<_, _>>();

    let res = dfs("svr", "out", &graph, &mut HashMap::new());
    println!("Res: {res}");

    let mut cache = HashMap::new();
    let left = dfs("svr", "dac", &graph, &mut cache)
        * dfs("dac", "fft", &graph, &mut cache)
        * dfs("fft", "out", &graph, &mut cache);

    let right = dfs("svr", "fft", &graph, &mut HashMap::new())
        * dfs("fft", "dac", &graph, &mut HashMap::new())
        * dfs("dac", "out", &graph, &mut HashMap::new());

    println!("Res[{},{}]: {}", left, right, left + right);
}
