use std::{fmt::Display, fs, io::Write, thread::sleep, time::Duration};

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

    fn load_robots() -> Vec<Robot> {
        let content = fs::read_to_string("input.txt")
            .expect("input.txt should be present in the root directory.");

        content
            .lines()
            .map(|line| {
                let (left_part, right_part) = line.split_once(" ").unwrap();
                let (_, position) = left_part.split_once("=").unwrap();
                let (_, velocity) = right_part.split_once("=").unwrap();

                let (x, y) = position.split_once(",").unwrap();
                let (x, y) = (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap());

                let (dx, dy) = velocity.split_once(",").unwrap();
                let (dx, dy) = (dx.parse::<isize>().unwrap(), dy.parse::<isize>().unwrap());

                Robot {
                    position: (y, x),
                    velocity: (dy, dx),
                }
            })
            .collect::<Vec<_>>()
    }
}

struct Grid {
    row: usize,
    col: usize,
    robots: Vec<Robot>,
    state: Vec<Vec<usize>>,
    safety_factor: usize,
    balanced_safety: bool,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.state {
            for &cell in row {
                if cell == 0 {
                    write!(f, " . ")?;
                } else {
                    write!(f, " {} ", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn init(row: usize, col: usize) -> Self {
        let robots = Robot::load_robots();
        let mut state = vec![vec![0 as usize; col]; row];

        for robot in &robots {
            let (x, y) = robot.position;
            state[x as usize][y as usize] += 1;
        }

        Grid {
            row,
            col,
            robots,
            state,
            safety_factor: 0,
            balanced_safety: false,
        }
    }

    fn arrangement(&mut self) {
        // Reset state.
        self.state = vec![vec![0 as usize; self.col]; self.row];

        for robot in &mut self.robots {
            robot.walk(self.row as isize, self.col as isize);
            let (x, y) = robot.position;
            self.state[x as usize][y as usize] += 1;
        }
    }

    fn arrangement_after(&mut self, second: usize) {
        for sec in 0..second {
            // Clean the terminal.
            print!("\x1B[2J\x1B[1;1H");
            std::io::stdout().flush().unwrap();
            println!("Checking grid configuration at {sec} second.");

            if self.balanced_safety && self.is_forming_tree() {
                println!("{}", self);
                sleep(Duration::from_secs(1));
            }

            self.arrangement();
            self.adjust_safety_factor();
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
        safety
    }

    fn adjust_safety_factor(&mut self) {
        let (x1, x2, x3) = (0, self.row / 2, self.row);
        let (y1, y2, y3) = (0, self.col / 2, self.col);

        let top_left_safety = self.quadrant_safety((x1, x2), (y1, y2));
        let bottom_left_safety = self.quadrant_safety((x1, x2), (y2 + 1, y3));
        let top_right_safety = self.quadrant_safety((x2 + 1, x3), (y1, y2));
        let bottom_right_safety = self.quadrant_safety((x2 + 1, x3), (y2 + 1, y3));

        self.safety_factor =
            top_left_safety * top_right_safety * bottom_left_safety * bottom_right_safety;

        self.balanced_safety =
            top_left_safety == top_right_safety && bottom_left_safety == bottom_right_safety;
    }

    fn is_forming_tree(&self) -> bool {
        for row in 0..self.row / 2 {
            for col in 0..self.col / 2 {
                if self.state[row][col] != self.state[self.row - 1 - row][col] {
                    return false;
                }
            }
        }

        for row in (self.row / 2) + 1.. {
            for col in self.col / 2 + 1.. {
                if self.state[row][col] != self.state[self.row - 1 - row][col] {
                    return false;
                }
            }
        }

        return true;
    }
}

pub fn robot_movement() {
    let mut grid = Grid::init(103, 101);
    grid.arrangement_after(1000000);
}
