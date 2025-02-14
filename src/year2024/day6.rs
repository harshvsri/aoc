use std::fs;

enum Dir {
    Up(isize, isize),
    Right(isize, isize),
    Down(isize, isize),
    Left(isize, isize),
}

impl Dir {
    fn default() -> Dir {
        Dir::Up(-1, 0)
    }

    fn change_dir(&mut self) {
        match *self {
            Dir::Up(_, _) => *self = Dir::Right(0, 1),
            Dir::Right(_, _) => *self = Dir::Down(1, 0),
            Dir::Down(_, _) => *self = Dir::Left(0, -1),
            Dir::Left(_, _) => *self = Dir::Up(-1, 0),
        }
    }

    fn get_dir(&self) -> (isize, isize) {
        match *self {
            Dir::Up(x, y) => (x, y),
            Dir::Right(x, y) => (x, y),
            Dir::Down(x, y) => (x, y),
            Dir::Left(x, y) => (x, y),
        }
    }
}

pub fn get_path() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut map_grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut dir = Dir::default();

    let start_pos = match get_start_index(&map_grid) {
        Some(val) => val,
        None => panic!("No guard (^) found."),
    };

    traverse(start_pos, &mut dir, &mut map_grid);

    for row in &map_grid {
        println!("{:?}", row)
    }

    let count = count_x(&map_grid);
    println!("X Count: {}", count);
}

fn traverse(pos: (isize, isize), dir: &mut Dir, map_grid: &mut Vec<Vec<char>>) {
    let (x, y) = pos;
    let (dx, dy) = dir.get_dir();
    let (mut next_x, mut next_y) = (x + dx, y + dy);
    map_grid[x as usize][y as usize] = 'X';

    if !is_valid_index(next_x, next_y, &map_grid) {
        return;
    }

    if map_grid[next_x as usize][next_y as usize] == '#' {
        dir.change_dir();
        let (dx, dy) = dir.get_dir();
        (next_x, next_y) = (x + dx, y + dy);
    };

    traverse((next_x, next_y), dir, map_grid);
}

fn is_valid_index(row: isize, col: isize, map_grid: &Vec<Vec<char>>) -> bool {
    row >= 0 && col >= 0 && (row as usize) < map_grid.len() && (col as usize) < map_grid[0].len()
}

fn get_start_index(map_grid: &Vec<Vec<char>>) -> Option<(isize, isize)> {
    for row in 0..map_grid.len() {
        for col in 0..map_grid[0].len() {
            if map_grid[row][col] == '^' {
                return Some((row as isize, col as isize));
            }
        }
    }
    return None;
}

fn count_x(map_grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in 0..map_grid.len() {
        for col in 0..map_grid[0].len() {
            if map_grid[row][col] == 'X' {
                count += 1;
            }
        }
    }
    return count;
}
