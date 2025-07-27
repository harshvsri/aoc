const RANGE: [f64; 2] = [200000000000000_f64, 400000000000000_f64];

struct HailStone {
    position: [f64; 3],
    velocity: [f64; 3],
    slope: f64,
    intercept: f64,
}

impl HailStone {
    fn parse(values: &str) -> [f64; 3] {
        let values = values
            .split(",")
            .map(|value| {
                value
                    .trim()
                    .parse::<f64>()
                    .expect("Must be a valid number.")
            })
            .collect::<Vec<_>>();

        [values[0], values[1], values[2]]
    }

    fn get_eqn((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> (f64, f64) {
        let slope = (y2 - y1) / (x2 - x1);
        let intercept = y1 - (slope * x1);
        (slope, intercept)
    }

    fn from_line(line: &str) -> Self {
        let (position, velocity) = line.split_once("@").expect("Must be a valid delemeter.");
        let (position, velocity) = (HailStone::parse(position), HailStone::parse(velocity));
        let (slope, intercept) = HailStone::get_eqn(
            (position[0], position[1]),
            (position[0] + velocity[0], position[1] + velocity[1]),
        );

        HailStone {
            position,
            velocity,
            slope,
            intercept,
        }
    }

    fn get_intersection(a: &HailStone, b: &HailStone) -> Option<(f64, f64)> {
        //println!("Hailstone A: {:?} @ {:?}", a.position, a.velocity);
        //println!("Hailstone B: {:?} @ {:?}", b.position, b.velocity);

        if a.slope == b.slope {
            //println!("Parallel or Considing.");
            return None;
        }

        let x = (b.intercept - a.intercept) / (a.slope - b.slope);
        let y = a.slope * x + a.intercept;

        if (x - a.position[0]) / a.velocity[0] < 0_f64
            || (x - b.position[0]) / b.velocity[0] < 0_f64
        {
            //println!("Intersects in the past.");
            return None;
        }

        if x >= RANGE[0] && x <= RANGE[1] && y >= RANGE[0] && y <= RANGE[1] {
            //println!("Intersects at ({x}, {y}).");
            return Some((x, y));
        }

        //println!("Intersects outside boundary at ({x}, {y}).");
        return None;
    }
}

pub fn foo() {
    let hailstones = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| HailStone::from_line(line))
        .collect::<Vec<_>>();

    let mut valid_intersections = 0;
    for l1 in 0..hailstones.len() {
        for l2 in l1 + 1..hailstones.len() {
            match HailStone::get_intersection(&hailstones[l1], &hailstones[l2]) {
                Some(_) => {
                    valid_intersections += 1;
                }
                None => {}
            }
        }
    }
    println!("Valid Intersections -> {valid_intersections}");
}
