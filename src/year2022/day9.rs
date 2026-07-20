#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

pub struct Game {
    moves: Vec<(u8, u8)>,
    rope: [Pos; 10],
}

impl Game {
    pub fn init(moves_str: &str) -> Self {
        fn parse_moves(moves_str: &str) -> Vec<(u8, u8)> {
            moves_str
                .lines()
                .map(|line| {
                    let (dir, count) = line
                        .split_once(' ')
                        .expect("Must contain a valid delemeter");
                    (
                        dir.as_bytes()[0],
                        count.parse::<u8>().expect("Must be a valid number"),
                    )
                })
                .collect::<Vec<_>>()
        }

        Self {
            moves: parse_moves(moves_str),
            rope: [Pos { x: 0, y: 0 }; 10],
        }
    }

    fn update_rope(&mut self, index: usize) {
        if index == self.rope.len() {
            return;
        }

        if self.rope[index].x.abs_diff(self.rope[index - 1].x) <= 1
            && self.rope[index].y.abs_diff(self.rope[index - 1].y) <= 1
        {
            return;
        }
        self.rope[index].x += (self.rope[index - 1].x - self.rope[index].x).signum();
        self.rope[index].y += (self.rope[index - 1].y - self.rope[index].y).signum();

        self.update_rope(index + 1);
    }

    pub fn play(&mut self) {
        fn dir_to_coord(dir: u8) -> (isize, isize) {
            match dir {
                b'U' => (0, 1),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                b'R' => (1, 0),
                _ => unreachable!(),
            }
        }

        let mut visited = std::collections::HashSet::from([self.rope[self.rope.len() - 1]]);
        for (dir, count) in std::mem::take(&mut self.moves) {
            let (dx, dy) = dir_to_coord(dir);
            for _ in 0..count {
                self.rope[0].x += dx;
                self.rope[0].y += dy;

                self.update_rope(1);
                visited.insert(self.rope[self.rope.len() - 1]);
            }
        }
        println!("Path len: {}", visited.len());
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    Game::init(&data).play();
}
