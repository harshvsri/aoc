const RANGE: [f64; 2] = [200000000000000_f64, 400000000000000_f64];

struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

struct HailStone {
    position: Coord,
    velocity: Coord,
}

impl HailStone {
    fn new(line: &str) -> Self {
        let (position, velocity) = line.split_once(" @ ").expect("Must be a valid delemeter.");
        let (position, velocity) = (
            HailStone::parse_coord(position),
            HailStone::parse_coord(velocity),
        );

        HailStone { position, velocity }
    }

    fn parse_coord(values: &str) -> Coord {
        let values = values
            .split(", ")
            .map(|value| value.parse::<f64>().expect("Must be a valid number."))
            .collect::<Vec<_>>();

        Coord {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }

    fn intersects(&self, other: &HailStone) -> bool {
        let (px1, py1) = (self.position.x, self.position.y);
        let (vx1, vy1) = (self.velocity.x, self.velocity.y);
        let (px2, py2) = (other.position.x, other.position.y);
        let (vx2, vy2) = (other.velocity.x, other.velocity.y);

        let determinant = vx1 * vy2 - vy1 * vx2;
        if determinant.abs() < f64::EPSILON {
            return false;
        }

        let (dx, dy) = (px2 - px1, py2 - py1);

        let t1 = (dx * vy2 - dy * vx2) / determinant;
        let t2 = (dx * vy1 - dy * vx1) / determinant;

        if t1 < 0.0 || t2 < 0.0 {
            return false;
        }

        let (x, y) = (px1 + vx1 * t1, py1 + vy1 * t1);
        x >= RANGE[0] && x <= RANGE[1] && y >= RANGE[0] && y <= RANGE[1]
    }
}

pub fn foo() {
    let hailstones = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| HailStone::new(line))
        .collect::<Vec<_>>();

    let mut valid_intersections = 0;
    for l1 in 0..hailstones.len() {
        for l2 in l1 + 1..hailstones.len() {
            if hailstones[l1].intersects(&hailstones[l2]) {
                valid_intersections += 1;
            }
        }
    }
    println!("Valid Intersections -> {valid_intersections}");
}
