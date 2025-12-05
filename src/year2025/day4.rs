const DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn is_valid(grid: &Vec<Vec<char>>, (x, y): (isize, isize)) -> bool {
    let mut count = 0;
    for (dx, dy) in DIRS {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || nx as usize >= grid.len() || ny < 0 || ny as usize >= grid[0].len() {
            continue;
        }
        if grid[nx as usize][ny as usize] == '@' {
            count += 1;
        }
    }

    count < 4
}

pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut grid = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut rolls = 0;
    let mut total_rolls = 0;
    let mut marked_for_removal = vec![];
    loop {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '@' && is_valid(&grid, (row as isize, col as isize)) {
                    rolls += 1;
                    marked_for_removal.push((row, col));
                }
            }
        }
        if rolls == 0 {
            break;
        }

        for &(r, c) in &marked_for_removal {
            grid[r][c] = '.';
        }
        total_rolls += rolls;
        rolls = 0;
    }
    println!("Total rolls: {total_rolls}");
}
