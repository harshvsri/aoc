use std::collections::HashMap;

const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn to_coords(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }
}

pub fn longest_hike() {
    let map = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let best = dense_longest_path(&map);
    println!("AI longest path: {best}");
}

fn dense_longest_path(map: &[Vec<char>]) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let start = (0usize, 1usize);
    let end = (rows - 1, cols - 2);

    let mut nodes = Vec::new();
    let mut id_for = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == '#' {
                continue;
            }
            // Junctions become explicit graph nodes.
            let mut neighbor_dirs = Vec::with_capacity(4);
            for dir in DIRS {
                let (dr, dc) = dir.to_coords();
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0
                    && nr < rows as isize
                    && nc >= 0
                    && nc < cols as isize
                    && map[nr as usize][nc as usize] != '#'
                {
                    neighbor_dirs.push((dr, dc));
                }
            }

            let is_junction = match neighbor_dirs.len() {
                0 | 1 | 3 | 4 => true,
                2 => {
                    let a = neighbor_dirs[0];
                    let b = neighbor_dirs[1];
                    a.0 + b.0 != 0 || a.1 + b.1 != 0
                }
                _ => false,
            };

            if (r, c) == start || (r, c) == end || is_junction {
                let id = nodes.len();
                nodes.push((r, c));
                id_for.insert((r, c), id);
            }
        }
    }

    let mut graph = vec![Vec::new(); nodes.len()];
    for (node_id, &(r, c)) in nodes.iter().enumerate() {
        for dir in &DIRS {
            let (dr, dc) = dir.to_coords();
            let (mut nr, mut nc) = (r as isize, c as isize);
            let mut dist = 0usize;
            loop {
                nr += dr;
                nc += dc;
                if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                    break;
                }
                if map[nr as usize][nc as usize] == '#' {
                    break;
                }
                dist += 1;
                // Corridors collapse into weighted edges between junctions.
                if let Some(&next_id) = id_for.get(&(nr as usize, nc as usize)) {
                    if next_id != node_id && !graph[node_id].iter().any(|(n, _)| *n == next_id) {
                        graph[node_id].push((next_id, dist));
                        graph[next_id].push((node_id, dist));
                    }
                    break;
                }
            }
        }
    }

    for edges in &mut graph {
        edges.sort_by(|a, b| b.1.cmp(&a.1));
    }

    // INFO:  At this point essentially we have found the junction nodes and their neighbors and stored then
    // with id label rather than position coords.

    let start_id = id_for[&start];
    let end_id = id_for[&end];
    let mut visited = vec![false; nodes.len()];
    let mut best = 0usize;

    fn dfs(
        node: usize,
        distance: usize,
        end: usize,
        graph: &Vec<Vec<(usize, usize)>>,
        visited: &mut Vec<bool>,
        best: &mut usize,
    ) {
        if node == end {
            *best = (*best).max(distance);
            return;
        }
        // Traverse without hashing for maximal throughput.
        for &(next, weight) in &graph[node] {
            if visited[next] {
                continue;
            }
            visited[next] = true;
            dfs(next, distance + weight, end, graph, visited, best);
            visited[next] = false;
        }
    }

    visited[start_id] = true;
    // dfs(start_id, 0, end_id, &graph, &mut visited, &mut best);
    best
}
