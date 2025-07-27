use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
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

fn get_start(map: &Vec<Vec<char>>) -> Option<(isize, isize)> {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                return Some((row as isize, col as isize));
            }
        }
    }
    return None;
}

const DIRS: &[Dir] = &[Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST];

pub fn get_lowest_score() {
    let map = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pq = BinaryHeap::from([(Reverse(0), get_start(&map).unwrap(), &Dir::EAST)]);
    let mut visited = HashSet::new();

    while !pq.is_empty() {
        let (score, (x, y), direction) = pq.pop().unwrap();
        if map[x as usize][y as usize] == 'E' {
            println!("Min score: {}.", score.0);
            break;
        }

        if visited.contains(&((x, y), direction)) {
            continue;
        }
        visited.insert(((x, y), direction));

        for dir in DIRS {
            let (dx, dy) = dir.to_coords();
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx == map.len() as isize || ny < 0 || ny == map[0].len() as isize {
                continue;
            }
            if map[nx as usize][ny as usize] == '#' {
                continue;
            }

            let nscore = if dir == direction { 1 } else { 1001 };
            pq.push((Reverse(score.0 + nscore), (x + dx, y + dy), dir));
        }
    }
}
