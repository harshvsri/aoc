use std::{cmp::min, collections::HashSet};

#[derive(PartialEq, Hash, Eq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_coords(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn complementary(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn traverse_grid(
    curr_pos: (isize, isize),
    curr_dir: &Direction,
    max_consecutive_moves: u8,
    visited: &mut HashSet<((isize, isize), Direction, u8)>,
    grid: &Vec<Vec<u8>>,
    dirs: &[Direction; 4],
) -> usize {
    let (x, y) = curr_pos;
    let mut min_heat = usize::MAX;

    if x < 0 || x == grid.len() as isize || y < 0 || y == grid[0].len() as isize {
        return min_heat;
    }
    if visited.contains(&((x, y), curr_dir.clone(), max_consecutive_moves)) {
        return min_heat;
    }
    visited.insert(((x, y), curr_dir.clone(), max_consecutive_moves));

    let curr_heat = grid[x as usize][y as usize] as usize;
    if x == grid.len() as isize - 1 && y == grid[0].len() as isize - 1 {
        return curr_heat;
    }

    for dir in dirs {
        let (dx, dy) = dir.to_coords();

        if (*dir == curr_dir.complementary()) || (dir == curr_dir && max_consecutive_moves == 0) {
            continue;
        }

        let new_max_consecutive_moves = if dir == curr_dir {
            max_consecutive_moves - 1
        } else {
            2
        };

        min_heat = min(
            min_heat,
            traverse_grid(
                (x + dx, y + dy),
                dir,
                new_max_consecutive_moves,
                visited,
                grid,
                dirs,
            ),
        );
    }

    visited.remove(&((x, y), curr_dir.clone(), max_consecutive_moves));
    if min_heat == usize::MAX {
        return min_heat;
    }
    return min_heat + curr_heat;
}

pub fn heat_loss() {
    let grid = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .expect(&format!("Must be a valid number. But got [{}]", ch))
                        as u8
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let dirs = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut visited_east = HashSet::new();
    let heat_from_east_start =
        traverse_grid((0, 1), &Direction::East, 2, &mut visited_east, &grid, &dirs);

    let mut visited_south = HashSet::new();
    let heat_from_south_start = traverse_grid(
        (1, 0),
        &Direction::South,
        2,
        &mut visited_south,
        &grid,
        &dirs,
    );

    let heat = min(heat_from_east_start, heat_from_south_start);
    println!("Heat -> {heat}");
}
