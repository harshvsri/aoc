const ROWS: usize = 103;
const COLS: usize = 101;

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn walk(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        self.position.0 = self.position.0.rem_euclid(ROWS as isize);
        self.position.1 = self.position.1.rem_euclid(COLS as isize);
    }

    fn load_robots() -> Vec<Robot> {
        let content = std::fs::read_to_string("input.txt")
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
    state: Vec<Vec<usize>>,
    robots: Vec<Robot>,
    safety_factor: usize,
}

impl std::fmt::Display for Grid {
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
    fn reset_state() -> Vec<Vec<usize>> {
        vec![vec![0usize; COLS]; ROWS]
    }
    fn init() -> Self {
        let robots = Robot::load_robots();
        let mut state = Grid::reset_state();

        for robot in &robots {
            let (x, y) = robot.position;
            state[x as usize][y as usize] += 1;
        }

        Grid {
            robots,
            state,
            safety_factor: 0,
        }
    }

    fn rearrange(&mut self) {
        self.state = Grid::reset_state();

        for robot in &mut self.robots {
            robot.walk();
            let (x, y) = robot.position;
            self.state[x as usize][y as usize] += 1;
        }
    }

    fn arrangement_after(&mut self) {
        let mut min_safety_factor = usize::MAX;
        let mut timestamp = 0;
        for time_elasped in 1..(ROWS * COLS) {
            self.rearrange();
            self.adjust_safety();
            if self.safety_factor < min_safety_factor {
                min_safety_factor = self.safety_factor;
                timestamp = time_elasped;
            }
        }
        println!("Minimum safety[{min_safety_factor}] at {timestamp}.");
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

    fn adjust_safety(&mut self) {
        let (x1, x2, x3) = (0, ROWS / 2, ROWS);
        let (y1, y2, y3) = (0, COLS / 2, COLS);

        let top_left_safety = self.quadrant_safety((x1, x2), (y1, y2));
        let bottom_left_safety = self.quadrant_safety((x1, x2), (y2 + 1, y3));
        let top_right_safety = self.quadrant_safety((x2 + 1, x3), (y1, y2));
        let bottom_right_safety = self.quadrant_safety((x2 + 1, x3), (y2 + 1, y3));

        self.safety_factor =
            top_left_safety * top_right_safety * bottom_left_safety * bottom_right_safety;
    }
}

pub fn robot_movement() {
    Grid::init().arrangement_after();
}
