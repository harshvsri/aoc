use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
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

fn wrap_coords(grid: &Vec<Vec<char>>, (mut x, mut y): (isize, isize)) -> (isize, isize) {
    let (max_x, max_y) = (grid.len() as isize, grid[0].len() as isize);
    (x, y) = (x % max_x, y % max_y);

    if x < 0 {
        x += max_x;
    }
    if y < 0 {
        y += max_y;
    }

    (x, y)
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

    let max_steps = (grid.len() / 2) + (0 * grid.len());
    let mut visited = HashMap::new();
    let mut pq = BinaryHeap::from([(Reverse(0), (x as isize, y as isize))]);

    while !pq.is_empty() {
        let (steps, (x, y)) = pq.pop().unwrap();
        if steps.0 > max_steps {
            break;
        }

        if visited.contains_key(&(x, y)) {
            continue;
        }
        visited.insert((x, y), steps.0);

        for dir in &dirs {
            let (dx, dy) = dir.to_coords();
            let (nx, ny) = (x + dx, y + dy);

            // Now we have an infinite map so we need to wrap around.
            let (nx_wrap, ny_wrap) = wrap_coords(&grid, (nx, ny));
            if grid[nx_wrap as usize][ny_wrap as usize] == '#' {
                continue;
            }
            pq.push((Reverse(steps.0 + 1), (nx, ny)));
        }
    }

    println!(
        "Steps[{max_steps}] -> Garden Plots[{}]",
        visited
            .iter()
            .filter(|(_, val)| **val % 2 == max_steps % 2)
            .count()
    );
}
