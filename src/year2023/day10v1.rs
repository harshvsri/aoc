#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn to_coords(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }

    fn get_valid_directions(c: char) -> Vec<Dir> {
        match c {
            'S' => vec![Dir::North, Dir::South, Dir::West, Dir::East],
            '|' => vec![Dir::North, Dir::South],
            '-' => vec![Dir::West, Dir::East],
            'F' => vec![Dir::South, Dir::East],
            '7' => vec![Dir::South, Dir::West],
            'L' => vec![Dir::North, Dir::East],
            'J' => vec![Dir::North, Dir::West],
            _ => vec![],
        }
    }

    fn is_valid(grid: &Vec<Vec<char>>, to: (isize, isize), from: (isize, isize)) -> bool {
        let (tx, ty) = to;
        let (fx, fy) = from;

        if tx < 0
            || tx == grid.len() as isize
            || ty < 0
            || ty == grid[0].len() as isize
            || grid[tx as usize][ty as usize] == '.'
        {
            return false;
        }

        let connecting_dirs = Dir::get_valid_directions(grid[tx as usize][ty as usize]);

        for dir in connecting_dirs {
            if (fx - tx, fy - ty) == dir.to_coords() {
                return true;
            }
        }
        return false;
    }
}

fn get_starting_tile(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                return Some((row, col));
            }
        }
    }
    return None;
}

fn traverse_grid(
    grid: &Vec<Vec<char>>,
    curr: (isize, isize),
    visited: &mut Vec<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let (x, y) = curr;
    let mut max_path = Vec::new();

    if grid[x as usize][y as usize] == 'S' && visited.len() >= 4 {
        return visited.clone();
    }

    if visited.contains(&(x, y)) {
        return vec![];
    }
    visited.push((x, y));

    for dir in Dir::get_valid_directions(grid[x as usize][y as usize]) {
        let (dx, dy) = dir.to_coords();
        let (new_x, new_y) = (x + dx, y + dy);

        if Dir::is_valid(grid, (new_x, new_y), (x, y)) {
            let path = traverse_grid(grid, (new_x, new_y), visited);
            if path.len() > max_path.len() {
                max_path = path;
            }
        }
    }

    visited.pop();
    max_path
}

fn get_enclosed_tiles(grid: &Vec<Vec<char>>, max_path: &Vec<(isize, isize)>) -> usize {
    let mut enclosed_tiles = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !max_path.contains(&(row as isize, col as isize)) {
                // Raycast check for each point.
                let mut boundary_crossed = 0;
                let (mut x, mut y) = (row as isize, col as isize);
                let (dx, dy) = Dir::East.to_coords();

                loop {
                    x += dx;
                    y += dy;
                    if x < 0 || x == grid.len() as isize || y < 0 || y == grid[0].len() as isize {
                        break;
                    }

                    if max_path.contains(&(x, y)) {
                        match grid[x as usize][y as usize] {
                            '|' => boundary_crossed += 1,
                            'J' => boundary_crossed += 1,
                            'L' => boundary_crossed += 1,
                            _ => {}
                        }
                    }
                }

                if boundary_crossed != 0 && boundary_crossed % 2 != 0 {
                    enclosed_tiles += 1;
                }
            }
        }
    }
    enclosed_tiles
}

pub fn get_steps() {
    let grid = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (x, y) = get_starting_tile(&grid).expect("Must contain a valid starting point.");
    let mut visited = Vec::new();

    let max_path = traverse_grid(&grid, (x as isize, y as isize), &mut visited);
    println!("Path({}) -> [...]", max_path.len());
    println!(
        "Enclosed points -> {}",
        get_enclosed_tiles(&grid, &max_path)
    );
    export_grid(&grid, &max_path);
}

fn export_grid(grid: &Vec<Vec<char>>, max_path: &Vec<(isize, isize)>) {
    let mut grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !max_path.contains(&(row as isize, col as isize)) {
                grid[row][col] = ' ';
            }
        }
    }

    let content = grid
        .iter()
        .map(|row| {
            let line = row.iter().collect::<String>();
            line.replace("F", "┌")
                .replace("7", "┐")
                .replace("L", "└")
                .replace("J", "┘")
                .replace("-", "─")
                .replace("|", "│")
        })
        .collect::<Vec<_>>();
    std::fs::write("puzzle.txt", content.join("\n")).unwrap();
}
