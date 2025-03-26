use std::{fmt::Display, fs};

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn walk(&mut self, max_row: isize, max_col: isize) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        //Adjusting wrap around.
        if self.position.0 < 0 {
            self.position.0 = max_row + self.position.0;
        }
        if self.position.0 > max_row - 1 {
            self.position.0 = self.position.0 % max_row;
        }

        if self.position.1 < 0 {
            self.position.1 = max_col + self.position.1;
        }
        if self.position.1 > max_col - 1 {
            self.position.1 = self.position.1 % max_col;
        }
    }
}

struct Grid {
    row: usize,
    col: usize,
    robots: Vec<Robot>,
    state: Vec<Vec<usize>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.state {
            for &cell in row {
                if cell == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn movement(&mut self) {
        self.state = vec![vec![0 as usize; self.col]; self.row];
        for robot in &mut self.robots {
            robot.walk(self.row as isize, self.col as isize);
            let (x, y) = robot.position;
            self.state[x as usize][y as usize] += 1;
        }
    }

    fn quadrant_safety(&self, row: (usize, usize), col: (usize, usize)) -> usize {
        let (row_start, row_end) = row;
        let (col_start, col_end) = col;

        let mut safety = 0;
        for row in row_start..row_end {
            for col in col_start..col_end {
                safety += self.state[row][col]
            }
        }
        println!("({:?},{:?}) -> {}", row, col, safety);
        safety
    }
}

pub fn get_safety_factor() {
    let mut grid = Grid {
        row: 103,
        col: 101,
        robots: get_robots(),
        state: vec![vec![0 as usize; 101]; 103],
    };

    println!("At the start.");
    println!("{}", grid);
    for _ in 0..100 {
        grid.movement();
    }
    println!("After 100 seconds.");
    println!("{}", grid);

    let (x1, x2, x3) = (0, grid.row / 2, grid.row);
    let (y1, y2, y3) = (0, grid.col / 2, grid.col);

    let mut total_safety = 1;
    total_safety *= grid.quadrant_safety((x1, x2), (y1, y2));
    total_safety *= grid.quadrant_safety((x1, x2), (y2 + 1, y3));
    total_safety *= grid.quadrant_safety((x2 + 1, x3), (y1, y2));
    total_safety *= grid.quadrant_safety((x2 + 1, x3), (y2 + 1, y3));
    println!("Total Safety: {total_safety}");
}

fn get_robots() -> Vec<Robot> {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    content
        .lines()
        .map(|line| {
            let (left_part, right_part) = line.split_once(" ").unwrap();
            let (_, position) = left_part.split_once("=").unwrap();
            let (_, velocity) = right_part.split_once("=").unwrap();

            let (x, y) = position.split_once(",").unwrap();
            let (x, y) = (parse_number(x), parse_number(y));

            let (dx, dy) = velocity.split_once(",").unwrap();
            let (dx, dy) = (parse_number(dx), parse_number(dy));

            Robot {
                position: (y, x),
                velocity: (dy, dx),
            }
        })
        .collect::<Vec<_>>()
}

fn parse_number(num_str: &str) -> isize {
    num_str.parse::<isize>().unwrap()
}
