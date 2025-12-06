use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Dir {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Dir {
    fn to_coords(&self) -> (isize, isize) {
        match self {
            Dir::NORTH => (-1, 0),
            Dir::EAST => (0, 1),
            Dir::SOUTH => (1, 0),
            Dir::WEST => (0, -1),
        }
    }
}

fn get_pos(map: &Vec<Vec<char>>, c: char) -> Option<(isize, isize)> {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == c {
                return Some((row as isize, col as isize));
            }
        }
    }
    return None;
}

fn make_path(
    prev_map: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    node: &(isize, isize),
    path: &mut Vec<(isize, isize)>,
) {
    if let Some(ps) = prev_map.get(node) {
        for &p in ps {
            path.push(p);
            make_path(prev_map, &p, path);
            path.pop();
        }
    } else {
        // We have reached to the start
        println!("Path[{}]: {:?}", path.len(), path.clone());
        return;
    }
}

const DIRS: &[Dir] = &[Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST];

pub fn solve() {
    let map = std::fs::read_to_string("test.txt")
        .expect("input.txt should be present in the root directory.");

    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = get_pos(&map, 'S').unwrap();
    let end = get_pos(&map, 'E').unwrap();
    let mut pq = BinaryHeap::from([(Reverse(0), start, &Dir::EAST)]);
    let mut score_map = HashMap::from([(start, 0)]);
    let mut prev_map = HashMap::new();

    while !pq.is_empty() {
        let (score, (x, y), direction) = pq.pop().unwrap();
        if (x, y) == end {
            println!("Min score: {}.", score.0);
            break;
        }

        for dir in DIRS {
            let (dx, dy) = dir.to_coords();
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx == map.len() as isize || ny < 0 || ny == map[0].len() as isize {
                continue;
            }
            if map[nx as usize][ny as usize] == '#' {
                continue;
            }

            let nscore = score.0 + if dir == direction { 1 } else { 1001 };
            match score_map.get_mut(&(nx, ny)) {
                None => {
                    score_map.insert((nx, ny), nscore);
                    prev_map.insert((nx, ny), vec![(x, y)]);
                    pq.push((Reverse(nscore), (nx, ny), dir));
                }
                Some(score) => {
                    if nscore == *score {
                        prev_map.entry((nx, ny)).or_insert(vec![]).push((x, y));
                        pq.push((Reverse(nscore), (nx, ny), dir));
                    }
                    if nscore < *score {
                        *score = nscore;
                        prev_map.insert((nx, ny), vec![(x, y)]);
                        pq.push((Reverse(nscore), (nx, ny), dir));
                    }
                }
            }
        }
    }

    make_path(&prev_map, &end, &mut vec![end]);
}
