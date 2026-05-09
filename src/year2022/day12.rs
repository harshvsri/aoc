const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub struct HillClimber {
    pub hill: Vec<Vec<char>>,
    pub start: (isize, isize),
    pub end: (isize, isize),
}

impl HillClimber {
    pub fn init() -> Self {
        fn find_pos(ch: char, map: &Vec<Vec<char>>) -> (isize, isize) {
            for r in 0..map.len() {
                for c in 0..map[0].len() {
                    if map[r][c] == ch {
                        return (r as isize, c as isize);
                    }
                }
            }
            unreachable!("Invalid char found.")
        }

        let mut hill = std::fs::read_to_string("input.txt")
            .expect("File must be present in the root directory.")
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = find_pos('S', &hill);
        let end = find_pos('E', &hill);

        // Clever optimizatiions for map handling.
        let (ex, ey) = end;
        hill[ex as usize][ey as usize] = ('z' as u8 + 1) as char;
        let (sx, sy) = start;
        hill[sx as usize][sy as usize] = 'a';

        Self { hill, start, end }
    }

    pub fn steps_to_reach_end(&self) -> u32 {
        let mut q = std::collections::BinaryHeap::from([(std::cmp::Reverse(0), self.start)]);
        let mut visited = vec![vec![u32::MAX; self.hill[0].len()]; self.hill.len()];
        let (x, y) = self.start;
        visited[x as usize][y as usize] = 0;

        while let Some((steps, (x, y))) = q.pop() {
            let steps = steps.0;

            if (x, y) == self.end {
                return steps;
            }

            for (dx, dy) in DIRS {
                let (nx, ny) = (x + dx, y + dy);

                // Skipping invalid next positions
                if nx < 0
                    || nx == self.hill.len() as isize
                    || ny < 0
                    || ny == self.hill[0].len() as isize
                {
                    continue;
                }

                // Allowing valid climbs omly, atmost 1 level at a time
                if self.hill[nx as usize][ny as usize] as u8
                    <= 1 + self.hill[x as usize][y as usize] as u8
                {
                    if visited[nx as usize][ny as usize] > steps + 1 {
                        visited[nx as usize][ny as usize] = steps + 1;
                        q.push((std::cmp::Reverse(steps + 1), (nx, ny)));
                    }
                }
            }
        }
        // No valid path found
        return u32::MAX;
    }

    pub fn explore_hill(&mut self) {
        let mut starting_points = vec![];
        for r in 0..self.hill.len() {
            for c in 0..self.hill[0].len() {
                if self.hill[r][c] == 'a' {
                    starting_points.push((r, c))
                }
            }
        }

        let min = starting_points
            .iter()
            .map(|&(x, y)| {
                self.start = (x as isize, y as isize);
                self.steps_to_reach_end()
            })
            .min()
            .expect("Must contain valid paths.");
        println!("Min path len: {min}");
    }
}

pub fn solve() {
    HillClimber::init().explore_hill();
}
