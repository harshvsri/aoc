use once_cell::sync::OnceCell;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

static MIN_SCORE: OnceCell<i32> = OnceCell::new();
const DIRS: &[Dir] = &[Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Dir {
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

    fn valid_dirs(&self) -> [Self; 3] {
        match self {
            Dir::NORTH => [Dir::NORTH, Dir::EAST, Dir::WEST],
            Dir::SOUTH => [Dir::EAST, Dir::SOUTH, Dir::WEST],
            Dir::EAST => [Dir::NORTH, Dir::EAST, Dir::SOUTH],
            Dir::WEST => [Dir::NORTH, Dir::SOUTH, Dir::WEST],
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

fn tile_count(
    end: (isize, isize),
    score_map: &HashMap<((isize, isize), Dir), i32>,
    prev_map: &HashMap<((isize, isize), Dir), Vec<((isize, isize), Dir)>>,
) -> usize {
    let mut q = Vec::new();
    let mut visited = HashSet::new();

    for &dir in DIRS {
        if let Some(&score) = score_map.get(&(end, dir))
            && &score == MIN_SCORE.get().expect("We never reached to the end node.")
        {
            q.push((end, dir));
            visited.insert((end, dir));
        }
    }

    while let Some(state) = q.pop() {
        if let Some(prevs) = prev_map.get(&state) {
            for &prev_state in prevs {
                if visited.insert(prev_state) {
                    q.push(prev_state);
                }
            }
        }
    }

    visited
        .iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

pub fn solve() {
    let map = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = get_pos(&map, 'S').unwrap();
    let end = get_pos(&map, 'E').unwrap();
    let mut pq = BinaryHeap::from([(Reverse(0), start, Dir::EAST)]);
    let mut score_map = HashMap::from([((start, Dir::EAST), 0)]);
    let mut prev_map = HashMap::new();

    while let Some((score, (x, y), direction)) = pq.pop() {
        if (x, y) == end {
            MIN_SCORE.get_or_init(|| score.0);
        }

        for dir in direction.valid_dirs() {
            let (dx, dy) = dir.to_coords();
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx == map.len() as isize || ny < 0 || ny == map[0].len() as isize {
                continue;
            }
            if map[nx as usize][ny as usize] == '#' {
                continue;
            }

            let nscore = score.0 + if dir == direction { 1 } else { 1001 };
            match score_map.get_mut(&((nx, ny), dir)) {
                None => {
                    score_map.insert(((nx, ny), dir), nscore);
                    prev_map.insert(((nx, ny), dir), vec![((x, y), direction)]);
                    pq.push((Reverse(nscore), (nx, ny), dir));
                }

                Some(best_nscore) => {
                    match nscore.cmp(best_nscore) {
                        std::cmp::Ordering::Greater => continue,
                        std::cmp::Ordering::Less => {
                            *best_nscore = nscore;
                            prev_map.insert(((nx, ny), dir), vec![((x, y), direction)]);
                            pq.push((Reverse(nscore), (nx, ny), dir));
                        }
                        std::cmp::Ordering::Equal => {
                            prev_map
                                .entry(((nx, ny), dir))
                                .or_default()
                                .push(((x, y), direction));
                            // Intentionally not pushing to pq again to avoid duplicate work
                        }
                    }
                }
            }
        }
    }

    println!(
        "Unique Tiles in Best Paths: {}",
        tile_count(end, &score_map, &prev_map)
    );
}
