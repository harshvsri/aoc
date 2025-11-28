use std::{collections::HashSet, fs};

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn fencing_cost() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = HashSet::with_capacity(rows * cols);
    let mut total_fencing_cost = 0;

    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let mut island = Vec::new();
                get_island(&grid, (row, col), &mut visited, &mut island);
                let boundaries = count_boundary(&grid, &island);

                // let perimeter = count_perimeter(&grid, &island);
                total_fencing_cost += island.len() * boundaries;
            }
        }
    }
    println!("Total cost: {}", total_fencing_cost);
}

#[inline]
fn is_valid(grid: &Vec<Vec<char>>, (row, col): (isize, isize)) -> bool {
    row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize
}

fn get_island(
    grid: &Vec<Vec<char>>,
    (row, col): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    island: &mut Vec<(usize, usize)>,
) {
    visited.insert((row, col));
    island.push((row, col));

    for &(dx, dy) in &DIRS {
        let (next_row, next_col) = (row as isize + dx, col as isize + dy);

        if is_valid(grid, (next_row, next_col)) {
            let (next_row, next_col) = (next_row as usize, next_col as usize);
            if !visited.contains(&(next_row, next_col))
                && grid[next_row][next_col] == grid[row][col]
            {
                get_island(grid, (next_row, next_col), visited, island)
            }
        }
    }
}

pub fn count_perimeter(grid: &Vec<Vec<char>>, island: &Vec<(usize, usize)>) -> usize {
    let mut perimeter = 0;

    for &(x, y) in island {
        let curr_val = grid[x][y];

        for &(dx, dy) in &DIRS {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0
                || nx >= grid.len() as isize
                || ny < 0
                || ny == grid[0].len() as isize
                || grid[nx as usize][ny as usize] != curr_val
            {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn count_boundary(grid: &Vec<Vec<char>>, island: &Vec<(usize, usize)>) -> usize {
    #[inline]
    fn is_boundary((x, y): (isize, isize), dir: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
        let (neighbor_dx, neighbor_dy) = match dir {
            (-1, 0) | (1, 0) => (0, -1),
            (0, -1) | (0, 1) => (-1, 0),
            _ => panic!("Invalid direction."),
        };

        let (dx, dy) = dir;
        let (along_dir_x, along_dir_y) = (x + dx, y + dy);
        let (neighbor_x, neighbor_y) = (x + neighbor_dx, y + neighbor_dy);
        let (neighbor_along_dir_x, neighbor_along_dir_y) = (neighbor_x + dx, neighbor_y + dy);

        let has_boundary = !is_valid(grid, (along_dir_x, along_dir_y))
            || grid[along_dir_x as usize][along_dir_y as usize] != grid[x as usize][y as usize];

        let has_neighbor = is_valid(grid, (neighbor_x, neighbor_y))
            && grid[neighbor_x as usize][neighbor_y as usize] == grid[x as usize][y as usize];

        let neighbor_has_boundary = !is_valid(grid, (neighbor_along_dir_x, neighbor_along_dir_y))
            || grid[neighbor_along_dir_x as usize][neighbor_along_dir_y as usize]
                != grid[x as usize][y as usize];

        has_boundary && !(has_neighbor && neighbor_has_boundary)
    }

    let mut boundary_count = 0;
    for dir in DIRS {
        for &(x, y) in island {
            if is_boundary((x as isize, y as isize), dir, grid) {
                boundary_count += 1;
            }
        }
    }
    boundary_count
}
