struct Warehouse {
    map: Vec<Vec<char>>,
    movements: Vec<char>,
    robot_pos: (isize, isize),
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for ch in row {
                write!(f, "{}", ch)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Warehouse {
    fn decode_movement(symbol: char) -> (isize, isize) {
        match symbol {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        }
    }

    fn init() -> Self {
        let content = std::fs::read_to_string("input.txt")
            .expect("input.txt should be present in the root directory.");

        let (map_str, movements_str) = content.split_once("\n\n").unwrap();

        let map_str = map_str
            .replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.");

        let map = map_str
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let movements = movements_str
            .lines()
            .flat_map(|l| l.chars())
            .collect::<Vec<_>>();

        let robot_pos = map
            .iter()
            .enumerate()
            .find_map(|(row, row_data)| {
                row_data
                    .iter()
                    .position(|&c| c == '@')
                    .map(|col| (row as isize, col as isize))
            })
            .expect("Map must have a robot as @.");

        Self {
            map,
            movements,
            robot_pos,
        }
    }

    fn can_move(&self, (x, y): (isize, isize), symbol: char) -> bool {
        let (dx, dy) = Self::decode_movement(symbol);
        let curr_symbol = self.map[x as usize][y as usize];
        let next_symbol = self.map[(x + dx) as usize][(y + dy) as usize];

        match next_symbol {
            '#' => false,
            '.' => true,
            '[' => match (dx, dy) {
                (-1, 0) | (1, 0) => {
                    self.can_move((x + dx, y), symbol) && self.can_move((x + dx, y + 1), symbol)
                }
                (0, -1) | (0, 1) => self.can_move((x, y + dy), symbol),
                _ => unreachable!("Invalid direction found [{x},{y}]."),
            },
            ']' => match (dx, dy) {
                (-1, 0) | (1, 0) => {
                    self.can_move((x + dx, y), symbol) && self.can_move((x + dx, y - 1), symbol)
                }
                (0, -1) | (0, 1) => self.can_move((x, y + dy), symbol),
                _ => unreachable!("Invalid direction found [{x},{y}]."),
            },
            _ => unreachable!("Got an unexpected symbol: {curr_symbol}"),
        }
    }

    fn move_robot(&mut self, (x, y): (isize, isize), symbol: char) {
        let (dx, dy) = Self::decode_movement(symbol);

        match self.map[(x + dx) as usize][(y + dy) as usize] {
            '[' => match (dx, dy) {
                (-1, 0) | (1, 0) => {
                    self.move_robot((x + dx, y), symbol);
                    self.move_robot((x + dx, y + 1), symbol)
                }
                (0, -1) | (0, 1) => self.move_robot((x, y + dy), symbol),
                _ => unreachable!("Invalid direction found [{x},{y}]."),
            },
            ']' => match (dx, dy) {
                (-1, 0) | (1, 0) => {
                    self.move_robot((x + dx, y), symbol);
                    self.move_robot((x + dx, y - 1), symbol)
                }
                (0, -1) | (0, 1) => self.move_robot((x, y + dy), symbol),
                _ => unreachable!("Invalid direction found [{x},{y}]."),
            },
            _ => {}
        }

        self.swap((x, y), (x + dx, y + dy));
    }

    fn swap(&mut self, a: (isize, isize), b: (isize, isize)) {
        assert!(self.map[b.0 as usize][b.1 as usize] == '.');

        let curr_symbol = self.map[a.0 as usize][a.1 as usize];
        self.map[a.0 as usize][a.1 as usize] = '.';
        self.map[b.0 as usize][b.1 as usize] = curr_symbol;
        if curr_symbol == '@' {
            // Update robot robot_position.
            self.robot_pos = b;
        }
    }

    fn run(&mut self) {
        for idx in 0..self.movements.len() {
            let (pos, symbol) = (self.robot_pos, self.movements[idx]);
            if self.can_move(pos, symbol) {
                self.move_robot(pos, symbol);
            }
        }
        println!("GPS Value: {}", self.compute_gps());
    }

    fn compute_gps(&self) -> usize {
        let mut gps_value = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == '[' {
                    gps_value += 100 * row + col;
                }
            }
        }
        gps_value
    }
}

pub fn process() {
    Warehouse::init().run();
}
