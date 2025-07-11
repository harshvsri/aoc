use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    io::{stdout, Write},
    usize,
};

#[derive(Debug)]
enum Axis {
    X,
    Y,
    Z,
}
#[derive(Debug)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
    axis: Axis,
}

impl Brick {
    fn parse_points(point: &str) -> (usize, usize, usize) {
        let point = point
            .split(",")
            .map(|x| x.parse::<usize>().expect("Must be a valid number."))
            .collect::<Vec<_>>();

        (point[0], point[1], point[2])
    }

    fn new(start: &str, end: &str) -> Self {
        let (x1, y1, z1) = Brick::parse_points(start);
        let (x2, y2, z2) = Brick::parse_points(end);

        let axis = if x1 != x2 {
            Axis::X
        } else if y1 != y2 {
            Axis::Y
        } else {
            Axis::Z
        };

        Brick {
            start: (x1, y1, z1),
            end: (x2, y2, z2),
            axis,
        }
    }

    fn to_cells(&self) -> Vec<(usize, usize, usize)> {
        let mut cells = vec![];

        match self.axis {
            Axis::X => {
                let (_, y, z) = self.start;
                for x in self.start.0..=self.end.0 {
                    cells.push((x, y, z));
                }
            }
            Axis::Y => {
                let (x, _, z) = self.start;
                for y in self.start.1..=self.end.1 {
                    cells.push((x, y, z));
                }
            }
            Axis::Z => {
                let (x, y, _) = self.start;
                for z in self.start.2..=self.end.2 {
                    cells.push((x, y, z));
                }
            }
        };
        cells
    }

    fn collides(&self, occupied_spaces: &HashMap<(usize, usize, usize), usize>) -> bool {
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                if occupied_spaces.contains_key(&(x, y, min(self.start.2, self.end.2) - 1)) {
                    return true;
                }
            }
        }
        return false;
    }

    fn free_fall(&mut self, occupied_spaces: &HashMap<(usize, usize, usize), usize>) {
        loop {
            // Brick has reached ground or hits a support brick.
            if self.start.2 == 1 || self.end.2 == 1 || self.collides(occupied_spaces) {
                break;
            }

            self.start.2 -= 1;
            self.end.2 -= 1;
        }
    }

    fn supports_to(&self, map: &HashMap<(usize, usize, usize), usize>) -> HashSet<usize> {
        let mut bricks = HashSet::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                let key = (x, y, max(self.start.2, self.end.2) + 1);

                if map.contains_key(&key) {
                    bricks.insert(map.get(&key).unwrap().clone());
                }
            }
        }
        bricks
    }

    fn supported_by(&self, map: &HashMap<(usize, usize, usize), usize>) -> HashSet<usize> {
        let mut bricks = HashSet::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                let key = (x, y, min(self.start.2, self.end.2) - 1);

                if map.contains_key(&key) {
                    bricks.insert(map.get(&key).unwrap().clone());
                }
            }
        }
        bricks
    }

    fn chain_reaction(
        index: usize,
        bricks: &Vec<Brick>,
        map: &HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        let mut falled_bricks = HashSet::from([index]);
        let mut queue = VecDeque::from([index]);

        while !queue.is_empty() {
            let brick_idx = queue.pop_front().unwrap();

            for supported_brick_idx in bricks[brick_idx].supports_to(&map) {
                let supporter_bricks = bricks[supported_brick_idx].supported_by(map);

                if supporter_bricks.is_subset(&falled_bricks) {
                    falled_bricks.insert(supported_brick_idx);
                    queue.push_back(supported_brick_idx);
                }
            }
        }

        print!("\rProcessing brick [{index}] ...");
        stdout().flush().unwrap();
        falled_bricks.len() - 1
    }
}

pub fn count_bricks() {
    let mut bricks = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").expect("Must have a valid delimeter.");
            Brick::new(start, end)
        })
        .collect::<Vec<_>>();
    bricks.sort_by_key(|brick| min(brick.start.2, brick.end.2));

    let mut cells_brick_map = HashMap::new();
    for index in 0..bricks.len() {
        let brick = &mut bricks[index];
        brick.free_fall(&cells_brick_map);

        brick.to_cells().iter().for_each(|cell| {
            cells_brick_map.insert(cell.clone(), index);
        });
    }

    let safe_bricks = bricks
        .iter()
        .filter(|brick| {
            brick
                .supports_to(&cells_brick_map)
                .iter()
                .all(|brick_idx| bricks[*brick_idx].supported_by(&cells_brick_map).len() >= 2)
        })
        .count();
    println!("Safe bricks -> {safe_bricks}");

    let falled_bricks = bricks
        .iter()
        .enumerate()
        .map(|(index, _)| Brick::chain_reaction(index, &bricks, &cells_brick_map))
        .sum::<usize>();
    println!("Falled bricks -> {falled_bricks}");
}
