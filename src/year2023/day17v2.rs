use std::cmp::{min, Reverse};
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
    let mut min_heat_grid = vec![vec![u32::MAX; 13]; 13];
    min_heat_grid[0][0] = 0;

    while !pq.is_empty() {
        let (heat, (x, y), curr_dir, rem_steps) = pq.pop().expect("Heap must not be empty");

        visited.insert(((x, y), curr_dir.clone(), rem_steps));
        min_heat_grid[x as usize][y as usize] = min(min_heat_grid[x as usize][y as usize], heat.0);

        if x == grid.len() as isize - 1 && y == grid[0].len() as isize - 1 {
            break;
        }

        for dir in &dirs {
            let (dx, dy) = dir.to_coords();
            let (x, y) = (x + dx, y + dy);
            let new_rem_steps = if *dir == curr_dir && rem_steps > 0 {
                rem_steps - 1
            } else {
                2
            };

            if *dir == curr_dir.complementary()
                || *dir == curr_dir && rem_steps == 0
                || visited.contains(&((x, y), dir.clone(), new_rem_steps))
                || x < 0
                || x == grid.len() as isize
                || y < 0
                || y == grid[0].len() as isize
            {
                continue;
            }

            pq.push((
                Reverse(grid[x as usize][y as usize] + heat.0),
                (x, y),
                dir.clone(),
                new_rem_steps,
            ));
        }
    }

    // Hepler Function.
    for row in min_heat_grid {
        let row = row
            .iter()
            .map(|val| {
                if *val == u32::MAX {
                    "    ".to_string()
                } else {
                    format!("{: <4}", val.to_string())
                }
            })
            .collect::<Vec<_>>();

        println!("{}", row.join(" "));
    }
}
