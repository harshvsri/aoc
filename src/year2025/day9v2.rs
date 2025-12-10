use std::cmp::{max, min};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(line: &str) -> Self {
        let (y, x) = line
            .split_once(",")
            .expect("Must contain a valid delemeter.");
        let (x, y) = (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap());
        Self { x, y }
    }

    fn area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn forms_valid_rectangle(&self, other: &Point, edges: &[(&Point, &Point)]) -> bool {
        if self == other {
            return false;
        }

        // HACK: Check for every boundary ranges
        edges.iter().all(|(start, end)| {
            max(self.x, other.x) <= min(start.x, end.x)
                || min(self.x, other.x) >= max(start.x, end.x)
                || max(self.y, other.y) <= min(start.y, end.y)
                || min(self.y, other.y) >= max(start.y, end.y)
        })
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let tiles = data.lines().map(Point::new).collect::<Vec<_>>();

    let edges = (0..tiles.len())
        .map(|i| (&tiles[i], &tiles[(i + 1) % tiles.len()]))
        .collect::<Vec<_>>();

    let mut max_area = 0;
    for p1 in &tiles {
        for p2 in &tiles {
            if p1.forms_valid_rectangle(p2, &edges) {
                max_area = max_area.max(p1.area(p2));
            }
        }
    }
    println!("Max area: {max_area}");
}
