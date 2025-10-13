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

    compress(&map);
}

fn compress(map: &Vec<Vec<char>>) {
    fn is_junction((x, y): (isize, isize), map: &Vec<Vec<char>>) -> bool {
        if map[x as usize][y as usize] == '#' {
            return false;
        }

        let mut valid_dirs = vec![];
        for dir in &DIRS {
            let (dx, dy) = dir.to_coords();
            let (nx, ny) = (x + dx, y + dy);

            if nx >= 0
                && nx < map.len() as isize
                && ny >= 0
                && ny < map[0].len() as isize
                && map[nx as usize][ny as usize] != '#'
            {
                valid_dirs.push((dx, dy));
            }
        }

        // Used a bit clever move.
        match valid_dirs.len() {
            0 | 1 | 3 | 4 => true,
            2 => {
                let (a, b) = (valid_dirs[0], valid_dirs[1]);
                a.0 + b.0 != 0 || a.1 + b.1 != 0
            }
            _ => panic!("Invalid number of directions, cant be more than 4"),
        }
    }

    fn find_neighbors(
        node: (isize, isize),
        map: &Vec<Vec<char>>,
        node_id_map: &HashMap<(isize, isize), usize>,
        adjacacncy_list: &mut Vec<Vec<(usize, usize)>>,
    ) {
        let node_id = node_id_map[&node];

        for dir in DIRS {
            let (mut x, mut y) = node;
            let (dx, dy) = dir.to_coords();
            let mut dist = 0;

            loop {
                (x, y) = (x + dx, y + dy);
                dist += 1;

                // Well here is a bit confusion around weather should i map in both directions
                // since wewill map for every node anyways.
                if x >= 0
                    && x < map.len() as isize
                    && y >= 0
                    && y < map[0].len() as isize
                    && map[x as usize][y as usize] != '#'
                {
                    if let Some(&id) = node_id_map.get(&(x, y)) {
                        adjacacncy_list[node_id].push((id, dist));
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    // INFO: Most of the optimization will come from avoiding hashing all togather.
    // I have verified this using hyperfine that calculation of this doesnt affect all.

    fn longest_path(
        curr_node_id: usize,
        curr_dist: usize,
        max_dist: &mut usize,
        end_node_id: usize,
        map: &Vec<Vec<char>>,
        adjacacncy_list: &Vec<Vec<(usize, usize)>>,
        visited: &mut Vec<bool>,
    ) {
        if curr_node_id == end_node_id {
            *max_dist = (*max_dist).max(curr_dist);
            return;
        }

        for &(neighbor, dist) in &adjacacncy_list[curr_node_id] {
            if visited[neighbor] {
                continue;
            }
            visited[neighbor] = true;
            longest_path(
                neighbor,
                curr_dist + dist,
                max_dist,
                end_node_id,
                map,
                adjacacncy_list,
                visited,
            );
            visited[neighbor] = false;
        }
    }

    // Find all the junction nodes.
    let mut nodes = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if is_junction((row as isize, col as isize), map) {
                nodes.push((row as isize, col as isize));
            }
        }
    }
    println!("Junction nodes: {:?}", nodes.len());
    let node_count = nodes.len();

    let node_id_map = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.clone(), index))
        .collect::<HashMap<_, _>>();

    let mut adjacacncy_list = vec![vec![]; node_count];
    for node in nodes {
        find_neighbors(node, map, &node_id_map, &mut adjacacncy_list);
    }
    println!("Adjacacncy list: {:?}", adjacacncy_list.len());

    let (start, end) = ((0, 1), (map.len() as isize - 1, map.len() as isize - 2));
    let (start_node_id, end_node_id) = (node_id_map[&start], node_id_map[&end]);
    let mut visited = vec![false; node_count];
    let mut max_dist = 0;

    longest_path(
        start_node_id,
        0,
        &mut max_dist,
        end_node_id,
        &map,
        &adjacacncy_list,
        &mut visited,
    );
    println!("Longest path: {max_dist}");
}
