use std::collections::{HashMap, HashSet};

const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

#[derive(PartialEq, Eq)]
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

            if nx >= 0 && nx < map.len() as isize && ny >= 0 && ny < map[0].len() as isize {
                if map[nx as usize][ny as usize] != '#' {
                    valid_dirs.push(dir);
                }
            }
        }

        match valid_dirs.len() {
            0 => false,
            1 | 3 | 4 => true,
            2 => {
                if valid_dirs.contains(&&Dir::North) && valid_dirs.contains(&&Dir::South)
                    || valid_dirs.contains(&&Dir::East) && valid_dirs.contains(&&Dir::West)
                {
                    false
                } else {
                    true
                }
            }
            _ => panic!("Invalid number of directions, cant be more than 4"),
        }
    }

    fn find_neighbors(
        node: (isize, isize),
        map: &Vec<Vec<char>>,
        nodes: &HashSet<(isize, isize)>,
        adjacacncy_list: &mut HashMap<(isize, isize), Vec<((isize, isize), usize)>>,
    ) {
        fn visit_direction(
            node: (isize, isize),
            dir: Dir,
            map: &Vec<Vec<char>>,
            nodes: &HashSet<(isize, isize)>,
            adjacacncy_list: &mut HashMap<(isize, isize), Vec<((isize, isize), usize)>>,
        ) {
            let (mut x, mut y) = node;
            let mut dist = 0;
            loop {
                let (dx, dy) = dir.to_coords();
                (x, y) = (x + dx, y + dy);
                dist += 1;

                if x >= 0
                    && x < map.len() as isize
                    && y >= 0
                    && y < map[0].len() as isize
                    && map[x as usize][y as usize] != '#'
                {
                    if nodes.contains(&(x, y)) {
                        adjacacncy_list
                            .entry(node)
                            .and_modify(|v| v.push(((x, y), dist)))
                            .or_insert(vec![((x, y), dist)]);
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        for dir in DIRS {
            visit_direction(node, dir, map, nodes, adjacacncy_list);
        }
    }

    fn longest_path(
        curr_node: (isize, isize),
        curr_dist: usize,
        map: &Vec<Vec<char>>,
        adjacacncy_list: &HashMap<(isize, isize), Vec<((isize, isize), usize)>>,
        visited: &mut HashSet<(isize, isize)>,
    ) -> usize {
        // Check if we have readhed destination.
        if curr_node.0 == map.len() as isize - 1 && curr_node.1 == map[0].len() as isize - 2 {
            return curr_dist;
        }
        if !visited.insert(curr_node) {
            return 0;
        }

        let mut max_path = 0;
        for (neighbor, dist) in adjacacncy_list.get(&curr_node).unwrap() {
            max_path = max_path.max(longest_path(
                *neighbor,
                curr_dist + dist,
                map,
                adjacacncy_list,
                visited,
            ));
        }

        visited.remove(&(curr_node));
        return max_path;
    }

    // Find all the junction nodes.
    let mut nodes = HashSet::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if is_junction((row as isize, col as isize), map) {
                nodes.insert((row as isize, col as isize));
            }
        }
    }
    println!("Junction nodes: {:?}", nodes.len());

    let mut adjacacncy_list = HashMap::new();
    for node in &nodes {
        find_neighbors(*node, map, &nodes, &mut adjacacncy_list);
    }
    println!("Adjacacncy list: {:?}", adjacacncy_list.len());

    let len = longest_path((0, 1), 0, &map, &adjacacncy_list, &mut HashSet::new());
    println!("Longest path: {len}");
}
