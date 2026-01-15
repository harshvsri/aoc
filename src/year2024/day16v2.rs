use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use once_cell::sync::OnceCell;

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

pub fn make_path(
    prev_map: &HashMap<((isize, isize), Dir), Vec<((isize, isize), Dir)>>,
    node: ((isize, isize), Dir),
    path: &mut Vec<((isize, isize), Dir)>,
) {
    if let Some(ps) = prev_map.get(&node) {
        for &p in ps {
            path.push(p.clone());
            make_path(prev_map, p, path);
            path.pop();
        }
    } else {
        // We have reached to the start
        // println!("Path[{}]: {:?}", path.len(), path.iter().rev().clone());
        println!("Path[{}]: [...]", path.len());
        return;
    }
}

const DIRS: &[Dir] = &[Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST];
static MIN_SCORE: OnceCell<i32> = OnceCell::new();

pub fn solve() {
    let map = std::fs::read_to_string("test.txt")
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

    while !pq.is_empty() {
        let (score, (x, y), direction) = pq.pop().unwrap();
        if (x, y) == end {
            MIN_SCORE.get_or_init(|| score.0);
            println!("...");
            // make_path(&prev_map, (end, direction), &mut vec![(end, direction)]);

            // println!(
            //     "{:?}",
            //     prev_map
            //         .values()
            //         .map(|v: &Vec<((isize, isize), Dir)>| v.len())
            //         .sum::<usize>()
            // );
            // break;
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
            if let Some(&min_score) = MIN_SCORE.get()
                && nscore > min_score
            {
                continue;
            }

            match score_map.get_mut(&((nx, ny), dir)) {
                None => {
                    score_map.insert(((nx, ny), dir), nscore);
                    prev_map.insert(((nx, ny), dir), vec![((x, y), direction)]);
                    pq.push((Reverse(nscore), (nx, ny), dir));
                }
                Some(score) => {
                    if nscore == *score {
                        prev_map
                            .entry(((nx, ny), dir))
                            .or_insert(vec![])
                            .push(((x, y), direction));
                        pq.push((Reverse(nscore), (nx, ny), dir));
                    }
                    if nscore < *score {
                        *score = nscore;
                        prev_map.insert(((nx, ny), dir), vec![((x, y), direction)]);
                        pq.push((Reverse(nscore), (nx, ny), dir));
                    }
                }
            }
        }
    }

    let _min_score = DIRS
        .iter()
        .map(|&dir| score_map.get(&(end, dir)).unwrap_or(&i32::MAX))
        .min()
        .unwrap();

    // for &dir in DIRS {
    //     if let Some(x) = score_map.get(&(end, dir))
    //         && x == min_score
    //     {
    //         make_path(&prev_map, (end, dir), &mut vec![(end, dir)]);
    //     }
    // }
}
