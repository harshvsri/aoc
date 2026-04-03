use std::collections::HashSet;

pub type Pos = (usize, usize);
const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn solve() {
    let puzzle =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let grid = puzzle
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn find_pos(ch: char, grid: &[Vec<char>]) -> Pos {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == ch {
                    return (row, col);
                }
            }
        }
        panic!("No such character found.")
    }

    let (start_pos, end_pos) = (find_pos('S', &grid), find_pos('E', &grid));

    fn explore(pos: Pos, end_pos: Pos, grid: &[Vec<char>], visited: &mut HashSet<(usize, usize)>) {
        let (x, y) = pos;
        let slope = grid[x][y] as u8;
        for (dx, dy) in DIRS {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx == grid.len() as isize || ny < 0 || ny == grid[0].len() as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if !visited.insert((nx, ny)) {
                continue;
            }
            let next_slope = grid[nx][ny] as u8;
            if next_slope - slope == 0 || next_slope - slope == 0 {
                // only allowed range we can take.
                explore((nx, ny), end_pos, grid, visited);
            }
        }
    }

    explore(start_pos, end_pos, &grid, &mut HashSet::new());
}
