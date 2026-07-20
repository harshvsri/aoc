pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let moves = Game::parse_moves(&data);
    Game::default().play(&moves);
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Default)]
pub struct Game {
    head: Pos,
    tail: Pos,
}

impl Game {
    pub fn parse_moves(s: &str) -> Vec<(u8, u8)> {
        s.lines()
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

    fn update_tail(&mut self) {
        if self.tail.x.abs_diff(self.head.x) <= 1 && self.tail.y.abs_diff(self.head.y) <= 1 {
            return;
        }
        self.tail.x += (self.head.x - self.tail.x).signum();
        self.tail.y += (self.head.y - self.tail.y).signum();
    }

    pub fn play(&mut self, moves: &[(u8, u8)]) {
        fn dir_to_coord(dir: u8) -> (isize, isize) {
            match dir {
                b'U' => (0, 1),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                b'R' => (1, 0),
                _ => unreachable!(),
            }
        }

        let mut visited = std::collections::HashSet::from([self.tail.clone()]);
        for &(dir, count) in moves {
            let (dx, dy) = dir_to_coord(dir);
            for _ in 0..count {
                self.head.x += dx;
                self.head.y += dy;

                self.update_tail();
                visited.insert(self.tail.clone());
            }
        }
        println!("Path len: {}", visited.len());
    }
}
