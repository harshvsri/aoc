use std::fs;

struct Warehouse {
    map: Vec<Vec<char>>,
    robot_pos: (usize, usize),
    robot_movements: Vec<char>,
}

impl Warehouse {
    fn new(map: Vec<Vec<char>>, robot_movements: Vec<char>) -> Self {
        let robot_pos = {
            let mut position = None;
            for row in 0..map.len() {
                for col in 0..map[0].len() {
                    if map[row][col] == '@' {
                        position = Some((row, col));
                        break;
                    }
                }
                if position.is_some() {
                    break;
                }
            }
            position
        }
        .expect("Map must have a robot as @.");

        Warehouse {
            map,
            robot_pos,
            robot_movements,
        }
    }

    fn decode_movement(symbol: &char) -> Option<(isize, isize)> {
        match *symbol {
            '<' => Some((0, -1)),
            '>' => Some((0, 1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None,
        }
    }

    fn display_map(&self) {
        for row in &self.map {
            for ch in row {
                print!("{} ", ch);
            }
            println!()
        }
        println!();
    }

    fn update_robot_pos(&mut self, (x, y): (usize, usize), symbol: &char) -> bool {
        let (dx, dy) = Self::decode_movement(symbol)
            .expect("Movement symbol must be among valid chars(<,>,^,v).");
        let (next_x, next_y) = (((x as isize) + dx) as usize, ((y as isize) + dy) as usize);

        let can_move = {
            match self.map[next_x][next_y] {
                '.' => true,
                'O' => self.update_robot_pos((next_x, next_y), symbol),
                '#' => false,
                _ => panic!("Invalid character present in map."),
            }
        };

        if can_move {
            let curr_symbol = self.map[x][y];
            self.map[x][y] = '.';
            self.map[next_x][next_y] = curr_symbol;
            if curr_symbol == '@' {
                // Update robot position.
                self.robot_pos = (next_x, next_y);
            }
            return true;
        }
        return false;
    }

    fn get_final_robot_pos(&mut self) {
        println!("Initial state");
        self.display_map();

        let movements = self.robot_movements.clone();
        for symbol in movements {
            let _ = self.update_robot_pos(self.robot_pos, &symbol);
        }

        println!("Final state");
        self.display_map();
    }

    fn get_gps_value(&self) -> usize {
        let mut gps_value = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == 'O' {
                    gps_value += 100 * row + col;
                }
            }
        }
        gps_value
    }
}

pub fn parse_data() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let (map, robot_movements) = content.split_once("\n\n").unwrap();

    let map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let robot_movements = robot_movements
        .lines()
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .collect::<Vec<_>>();

    let mut warehouse = Warehouse::new(map, robot_movements);
    warehouse.get_final_robot_pos();
    println!("GPS Value: {}", warehouse.get_gps_value());
}
