use once_cell::sync::Lazy;
use std::{collections::HashSet, sync::Mutex};

static GLOBAL_SET: Lazy<Mutex<HashSet<((isize, isize), Direction)>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_coords(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

struct LightRay {
    pos: (isize, isize),
    dir: Direction,
    child_rays: Vec<LightRay>,
}

impl LightRay {
    fn is_valid(&self, grid: &Vec<Vec<char>>) -> bool {
        let (x, y) = self.pos;
        self.child_rays.is_empty()
            && x >= 0
            && x < grid.len() as isize
            && y >= 0
            && y < grid[0].len() as isize
    }

    fn adjust_pos((x, y): (isize, isize), dir: Direction) -> (isize, isize) {
        let (dx, dy) = dir.to_coords();
        (x + dx, y + dy)
    }

    fn change_dir(&mut self, grid: &Vec<Vec<char>>) {
        let mirror = grid[self.pos.0 as usize][self.pos.1 as usize];

        match mirror {
            '/' => match self.dir {
                Direction::North => self.dir = Direction::East,
                Direction::East => self.dir = Direction::North,
                Direction::South => self.dir = Direction::West,
                Direction::West => self.dir = Direction::South,
            },
            '\\' => match self.dir {
                Direction::North => self.dir = Direction::West,
                Direction::East => self.dir = Direction::South,
                Direction::South => self.dir = Direction::East,
                Direction::West => self.dir = Direction::North,
            },
            '|' => {
                if self.dir == Direction::West || self.dir == Direction::East {
                    let mut ray1 = LightRay {
                        pos: LightRay::adjust_pos(self.pos, Direction::North),
                        dir: Direction::North,
                        child_rays: vec![],
                    };
                    ray1.travel(grid);

                    let mut ray2 = LightRay {
                        pos: LightRay::adjust_pos(self.pos, Direction::South),
                        dir: Direction::South,
                        child_rays: vec![],
                    };
                    ray2.travel(grid);

                    self.child_rays.push(ray1);
                    self.child_rays.push(ray2);
                }
            }
            '-' => {
                if self.dir == Direction::North || self.dir == Direction::South {
                    let mut ray1 = LightRay {
                        pos: LightRay::adjust_pos(self.pos, Direction::West),
                        dir: Direction::West,
                        child_rays: vec![],
                    };
                    ray1.travel(grid);

                    let mut ray2 = LightRay {
                        pos: LightRay::adjust_pos(self.pos, Direction::East),
                        dir: Direction::East,
                        child_rays: vec![],
                    };
                    ray2.travel(grid);

                    self.child_rays.push(ray1);
                    self.child_rays.push(ray2);
                }
            }
            _ => {}
        }
    }

    fn travel(&mut self, grid: &Vec<Vec<char>>) {
        loop {
            let (x, y) = self.pos;
            if GLOBAL_SET
                .lock()
                .unwrap()
                .contains(&(self.pos, self.dir.clone()))
                || !self.is_valid(grid)
            {
                break;
            }

            GLOBAL_SET
                .lock()
                .unwrap()
                .insert((self.pos, self.dir.clone()));

            self.change_dir(grid);
            let (dx, dy) = self.dir.to_coords();
            self.pos = (x + dx, y + dy);
        }
    }

    fn emulate(pos: (isize, isize), dir: Direction, grid: &Vec<Vec<char>>) -> usize {
        GLOBAL_SET.lock().unwrap().clear();

        LightRay {
            pos,
            dir,
            child_rays: vec![],
        }
        .travel(&grid);

        let energized_tiles = GLOBAL_SET
            .lock()
            .unwrap()
            .iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len();

        println!("Energized Tiles({:?}) -> {energized_tiles}", pos);
        energized_tiles
    }
}

pub fn count_energized_tiles() {
    let grid = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max_energized_tiles = 0;

    for col in 0..grid[0].len() as isize {
        max_energized_tiles =
            max_energized_tiles.max(LightRay::emulate((0, col), Direction::South, &grid));

        max_energized_tiles = max_energized_tiles.max(LightRay::emulate(
            (grid.len() as isize - 1, col),
            Direction::North,
            &grid,
        ));
    }

    for row in 0..grid.len() as isize {
        max_energized_tiles =
            max_energized_tiles.max(LightRay::emulate((row, 0), Direction::East, &grid));

        max_energized_tiles = max_energized_tiles.max(LightRay::emulate(
            (row, grid[0].len() as isize - 1),
            Direction::West,
            &grid,
        ));
    }

    println!("\nMax Energized Tiles -> {max_energized_tiles}");
}
