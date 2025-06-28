use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Hash, Clone, PartialOrd, Eq, Ord)]
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

pub fn get_heat_loss() {
    let grid = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .expect(&format!("Must be a valid number. But got [{}]", ch))
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

    let mut pq = BinaryHeap::from([
        (Reverse(grid[0][1]), (0, 1), Direction::East, 2),
        (Reverse(grid[1][0]), (1, 0), Direction::South, 2),
    ]);
    let mut visited = HashSet::new();

    while let Some((heat, (x, y), curr_dir, rem_steps)) = pq.pop() {
        if !visited.insert(((x, y), curr_dir.clone(), rem_steps)) {
            continue;
        }

        if x == grid.len() as isize - 1 && y == grid[0].len() as isize - 1 {
            println!("Heat incurred({x},{y}) -> {}", heat.0);
            break;
        }

        for dir in &dirs {
            let (dx, dy) = dir.to_coords();
            let (new_x, new_y) = (x + dx, y + dy);
            let new_rem_steps = if *dir == curr_dir && rem_steps > 0 {
                rem_steps - 1
            } else {
                2
            };

            if *dir == curr_dir.complementary()
                || *dir == curr_dir && rem_steps == 0
                || new_x < 0
                || new_x == grid.len() as isize
                || new_y < 0
                || new_y == grid[0].len() as isize
            {
                continue;
            }

            pq.push((
                Reverse(grid[new_x as usize][new_y as usize] + heat.0),
                (new_x, new_y),
                dir.clone(),
                new_rem_steps,
            ));
        }
    }
}
