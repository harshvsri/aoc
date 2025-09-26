#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn default() -> Dir {
        Dir::Up
    }

    fn to_coords(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
        }
    }

    fn change_dir(&mut self) {
        match *self {
            Dir::Up => *self = Dir::Right,
            Dir::Right => *self = Dir::Down,
            Dir::Down => *self = Dir::Left,
            Dir::Left => *self = Dir::Up,
        }
    }
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

fn is_valid_pos(row: isize, col: isize, grid: &Vec<Vec<char>>) -> bool {
    row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len()
}

fn contains_loop(mut pos: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
    let mut dir = Dir::default();
    let mut path = std::collections::HashSet::new();

    loop {
        let (x, y) = pos;
        let (dx, dy) = dir.to_coords();

        if path.contains(&((x, y), dir.clone())) {
            return true;
        }
        path.insert(((x, y), dir.clone()));

        let (mut next_x, mut next_y) = (x + dx, y + dy);
        if !is_valid_pos(next_x, next_y, &grid) {
            break;
        }

        while grid[next_x as usize][next_y as usize] == '#' {
            dir.change_dir();
            let (dx, dy) = dir.to_coords();
            (next_x, next_y) = (x + dx, y + dy);
        }

        (pos.0, pos.1) = (next_x, next_y);
    }
    return false;
}

pub fn get_path() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let pos = match get_start_index(&grid) {
        Some(val) => val,
        None => panic!("No guard (^) found."),
    };

    let mut loop_count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' {
                grid[row][col] = '#';
                match contains_loop(pos, &grid) {
                    true => loop_count += 1,
                    false => {}
                };
                grid[row][col] = '.'
            }
        }
    }
    println!("Loop count: {loop_count}");
}
