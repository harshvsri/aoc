pub struct State {
    pub grid: Vec<Vec<char>>,
    pub drop_pont: (usize, usize),
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "Drop Point: {:?}", self.drop_pont)?;
        writeln!(f, "{}*", "  ".repeat(10 + self.drop_pont.1))?;
        for row in &self.grid {
            write!(f, "{}", "  ".repeat(10))?;
            for item in row {
                write!(f, "{} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl State {
    pub const MOVES: [(isize, isize); 3] = [(1, 0), (1, -1), (1, 1)];

    pub fn init(s: String) -> Self {
        let (mut min_col, mut max_col) = (usize::MAX, usize::MIN);
        let (min_row, mut max_row) = (usize::MAX, usize::MIN);

        let rock_paths = s
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|point| {
                        let (col, row) = point
                            .split_once(",")
                            .expect("Must contain a valid delimeter");
                        let (col, row) =
                            (col.parse::<usize>().unwrap(), row.parse::<usize>().unwrap());
                        max_row = max_row.max(row);
                        min_col = min_col.min(col);
                        max_col = max_col.max(col);
                        (row, col)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = (max_row - min_row) + 1;
        let cols = (max_col - min_col) + 1;

        let mut grid = vec![vec!['.'; cols]; rows];
        for rock_path in rock_paths {
            for points in rock_path.windows(2) {
                let (x1, y1) = points[0];
                let (x2, y2) = points[1];

                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid[x1][y - min_col] = '#';
                    }
                } else if y1 == y2 {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid[x][y1 - min_col] = '#';
                    }
                }
            }
        }

        Self {
            grid,
            drop_pont: (0, 500 - min_col),
        }
    }

    pub fn simulate(&mut self) {
        let mut sand_count = 0;
        loop {
            if !State::place_sand(&mut self.grid, self.drop_pont) {
                println!("Now we cant add more than [{sand_count}] sands.");
                println!("\n{self}");
                return;
            } else {
                sand_count += 1;
            }
        }
    }

    fn place_sand(grid: &mut Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
        for (dx, dy) in Self::MOVES {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            // This has logic for the abyss
            if nx < 0 || nx == grid.len() as isize || ny < 0 || ny == grid[0].len() as isize {
                return false;
            }

            if grid[nx as usize][ny as usize] == '#' || grid[nx as usize][ny as usize] == 'o' {
                // We come to the end due to rock or sand
                continue;
            }

            return Self::place_sand(grid, (nx as usize, ny as usize));
        }
        // Now we can place the sand
        grid[x][y] = 'o';
        return true;
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("test.txt").expect("File must be present in the root directory.");

    let mut state = State::init(data);
    state.simulate();
}
