use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    usize,
};

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
}

fn get_starting_point(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                return Some((row, col));
            }
        }
    }
    return None;
}

pub fn count_garden_plots() {
    let grid = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (x, y) = get_starting_point(&grid).expect("Grid must contain a starting point.");
    let dirs = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut visited = HashSet::new();
    let mut cost_grid = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut pq = BinaryHeap::from([(Reverse(0), (x as isize, y as isize))]);

    while !pq.is_empty() {
        let (cost, (x, y)) = pq.pop().unwrap();
        if cost.0 > 64 {
            break;
        }

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        cost_grid[x as usize][y as usize] = cost.0;

        for dir in &dirs {
            let (dx, dy) = dir.to_coords();
            let (nx, ny) = (x + dx, y + dy);

            // Now we have an infinite map so we need to wrap around.
            if nx < 0
                || nx == grid.len() as isize
                || ny < 0
                || ny == grid.len() as isize
                || grid[nx as usize][ny as usize] == '#'
            {
                continue;
            }
            pq.push((Reverse(cost.0 + 1), (nx, ny)));
        }
    }

    let mut tiles_count = 0;
    for row in &cost_grid {
        println!(
            "{}",
            row.iter()
                .map(|val| {
                    if *val == usize::MAX || !(*val).is_multiple_of(2) {
                        format!("{:>3}", "".to_string())
                    } else {
                        tiles_count += 1;
                        format!("{:>3}", val.to_string())
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    println!("Total tiles -> {tiles_count}");
}
