use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn get_direction(value: (isize, isize)) -> Self {
        match value {
            (-1, 0) => Direction::NORTH,
            (0, 1) => Direction::EAST,
            (1, 0) => Direction::SOUTH,
            (0, -1) => Direction::WEST,
            _ => panic!("Invalid value passed."),
        }
    }
}

fn find_char(map: &Vec<Vec<char>>, symbol: char) -> Option<(usize, usize)> {
    let mut index = None;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == symbol {
                index = Some((row, col));
            }
        }
    }
    index
}

fn traverse_map(
    pos: (usize, usize),
    dir: &Direction,
    map: &Vec<Vec<char>>,
    dirs: &[(isize, isize)],
    visited: &mut HashSet<(usize, usize)>,
    score: usize,
    best_score: &mut Option<usize>,
    grid_score: &mut HashMap<(usize, usize), usize>,
) {
    let (x, y) = pos;
    if visited.contains(&(x, y)) || map[x][y] == '#' {
        return;
    }

    if map[x][y] == 'E' {
        if let Some(s) = *best_score {
            *best_score = Some(s.min(score));
        } else {
            *best_score = Some(score);
        }
        return;
    }

    match grid_score.get_mut(&(x, y)) {
        Some(prev_score) => {
            if score >= *prev_score {
                return;
            } else {
                *prev_score = score;
            }
        }
        None => {
            grid_score.insert((x, y), score);
        }
    }

    visited.insert((x, y));

    for &(dx, dy) in dirs {
        let (nx, ny) = (((x as isize + dx) as usize), ((y as isize + dy) as usize));
        let next_dir = Direction::get_direction((dx, dy));

        let (new_dir, new_score) = if *dir != next_dir {
            (&Direction::get_direction((dx, dy)), score + 1000 + 1)
        } else {
            (dir, score + 1)
        };

        traverse_map(
            (nx, ny),
            new_dir,
            map,
            dirs,
            visited,
            new_score,
            best_score,
            grid_score,
        );
    }

    visited.remove(&(x, y));
}

pub fn get_lowest_score() {
    let map = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_char(&map, 'S').expect("Map must contain starting position as S.");
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::new();
    let mut best_score = None;
    let mut grid_score = HashMap::new();

    traverse_map(
        start,
        &Direction::EAST,
        &map,
        &dirs,
        &mut visited,
        0,
        &mut best_score,
        &mut grid_score,
    );

    println!("Min scores: {:?}", best_score.unwrap());
}
