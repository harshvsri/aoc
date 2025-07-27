use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

const MEM_SIZE: usize = 71;
const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

struct Memory {
    grid: [[char; MEM_SIZE]; MEM_SIZE],
    corrupted_coords: Vec<(usize, usize)>,
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            let _ = f.write_str(&format!("{:?}\n", row));
        }
        Ok(())
    }
}

impl Memory {
    fn init(corrupted_str: &str) -> Self {
        Memory {
            grid: [['.'; MEM_SIZE]; MEM_SIZE],
            corrupted_coords: corrupted_str
                .trim()
                .lines()
                .map(|line| {
                    let (y, x) = line
                        .split_once(",")
                        .expect("Must contain a valid seperator.");
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>(),
        }
    }

    fn has_path(&self) -> bool {
        let mut pq = BinaryHeap::from([(Reverse(0), (0, 0))]);
        let mut visited = HashSet::new();

        while !pq.is_empty() {
            let (cost, (x, y)) = pq.pop().unwrap();
            if x == MEM_SIZE as isize - 1 && y == MEM_SIZE as isize - 1 {
                return true;
            }

            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            for (dx, dy) in DIRS {
                let (nx, ny) = (x + dx, y + dy);

                if nx < 0
                    || nx == MEM_SIZE as isize
                    || ny < 0
                    || ny == MEM_SIZE as isize
                    || self.grid[nx as usize][ny as usize] == '#'
                {
                    continue;
                }
                pq.push((Reverse(cost.0 + 1), (nx, ny)));
            }
        }
        return false;
    }

    fn find_corrupted_coord(&mut self) {
        for idx in 0..self.corrupted_coords.len() {
            let (x, y) = self.corrupted_coords[idx];

            self.grid[x][y] = '#';
            if !self.has_path() {
                println!("Corrupted Coord -> {y},{x}");
                break;
            }
        }
    }
}

pub fn foo() {
    let corrupted_str = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");
    Memory::init(&corrupted_str).find_corrupted_coord();
}
