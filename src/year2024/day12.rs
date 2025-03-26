use std::{collections::HashSet, fs};

pub fn fencing_cost() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (rows, cols) = (grid.len(), grid[0].len());
    let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::with_capacity(rows * cols);

    let mut total_fencing_cost = 0;
    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let mut island = Vec::new();
                get_island(&grid, &dirs, (row, col), &mut visited, &mut island);
                let perimeter = count_perimeter(&grid, &island, &dirs);
                println!("[{}] -> {}X{}", grid[row][col], island.len(), perimeter,);
                total_fencing_cost += island.len() * perimeter;
            }
        }
    }
    println!("Total cost: {}", total_fencing_cost);
}

fn get_island(
    grid: &Vec<Vec<char>>,
    dirs: &[(isize, isize)],
    (row, col): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    island: &mut Vec<(usize, usize)>,
) {
    visited.insert((row, col));
    island.push((row, col));

    for &(dx, dy) in dirs {
        let (next_row, next_col) = (row as isize + dx, col as isize + dy);

        if is_valid(grid, (next_row, next_col)) {
            let (next_row, next_col) = (next_row as usize, next_col as usize);
            if !visited.contains(&(next_row, next_col))
                && grid[next_row][next_col] == grid[row][col]
            {
                get_island(grid, dirs, (next_row, next_col), visited, island)
            }
        }
    }
}

fn count_perimeter(
    grid: &Vec<Vec<char>>,
    island: &Vec<(usize, usize)>,
    dirs: &[(isize, isize)],
) -> usize {
    let mut perimeter = 0;

    for &(x, y) in island {
        let curr_val = grid[x][y];

        for &(dx, dy) in dirs {
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

fn is_valid(grid: &Vec<Vec<char>>, (row, col): (isize, isize)) -> bool {
    row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize
}
