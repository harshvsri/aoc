use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(PartialEq)]
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

    fn get_dir(dir: &str) -> Self {
        match dir {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => panic!("Invalid direction found."),
        }
    }
}

pub fn get_total_lava() {
    let steps = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .trim()
        .lines()
        .map(|line| {
            let info = line.split_whitespace().collect::<Vec<_>>();
            assert!(info.len() == 3);
            (
                Direction::get_dir(info[0]),
                info[1].parse::<usize>().expect("Must be a valid number."),
            )
        })
        .collect::<Vec<_>>();

    let mut boundary_points = HashSet::new();
    let (mut x, mut y) = (0, 0);
    for (dir, value) in &steps {
        let (dx, dy) = dir.to_coords();
        for _ in 0..*value {
            (x, y) = (x + dx, y + dy);
            boundary_points.insert((x, y));
        }
    }
    println!("Total Boundary Points -> {}", boundary_points.len());

    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for (x, y) in &boundary_points {
        min_x = min(min_x, *x);
        max_x = max(max_x, *x);
        min_y = min(min_y, *y);
        max_y = max(max_y, *y);
    }
    println!("X[{min_x}, {max_x}] -> Y[{min_y}, {max_y}]");

    let mut internal_points = 0;
    for row in min_x..=max_x {
        let mut is_inside = false;
        for col in min_y..=max_y {
            if boundary_points.contains(&(row, col)) {
                if boundary_points.contains(&(row - 1, col)) {
                    is_inside = !is_inside;
                }
            } else {
                if is_inside {
                    internal_points += 1;
                }
            }
        }
    }
    println!("Total Lava -> {}", boundary_points.len() + internal_points);
}
