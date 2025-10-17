use nalgebra::{Matrix6, Vector6};

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

    fn position_throw_obliterate(hailstones: &[HailStone]) -> i64 {
        // We need to find a rock's position (rx, ry, rz) and velocity (rvx, rvy, rvz)
        // such that it collides with ALL hailstones at some times t_i.
        // Taking only the first 3 hailstones gives us exactly 6 equations for 6 unknowns.

        let (h0, h1, h2) = (&hailstones[0], &hailstones[1], &hailstones[2]);

        // The collision condition for hailstone i at time t_i is:
        // rock_position + t_i * rock_velocity = hailstone_i_position + t_i * hailstone_i_velocity
        // Rearranging: (rock_position - hailstone_i_position) = t_i * (hailstone_i_velocity - rock_velocity)
        // This means the relative position vector is parallel to the relative velocity vector.
        // Cross product of parallel vectors = 0, so:
        // (rock_position - hailstone_i_position) × (hailstone_i_velocity - rock_velocity) = 0

        // Expanding the cross product and eliminating t_i gives us 3 equations per hailstone pair.
        // Subtracting equations for h0 and h1 (to eliminate quadratic terms in unknowns):
        // We get a linear system: A * [rx, ry, rz, rvx, rvy, rvz]^T = b

        // Construct the 6×6 coefficient matrix A.
        // Each pair of hailstones contributes 3 rows (one per dimension: x, y, z).
        // The pattern comes from: (p0 - p1) × (v - v0) = (p0 - p1) × (v - v1)
        // Rearranging: (p0 - p1) × v = (p0 - p1) × v0 - (p0 - p1) × v1
        // And similarly for the rock position r.

        let data = [
            // Row 0: From x-component of (h0-h1) cross product equation
            // Coefficients for [rx, ry, rz, rvx, rvy, rvz]
            0.0,
            h0.velocity.z - h1.velocity.z,
            h1.velocity.y - h0.velocity.y,
            0.0,
            h1.position.z - h0.position.z,
            h0.position.y - h1.position.y,
            // Row 1: From y-component of (h0-h1) cross product equation
            h1.velocity.z - h0.velocity.z,
            0.0,
            h0.velocity.x - h1.velocity.x,
            h0.position.z - h1.position.z,
            0.0,
            h1.position.x - h0.position.x,
            // Row 2: From z-component of (h0-h1) cross product equation
            h0.velocity.y - h1.velocity.y,
            h1.velocity.x - h0.velocity.x,
            0.0,
            h1.position.y - h0.position.y,
            h0.position.x - h1.position.x,
            0.0,
            // Row 3: From x-component of (h0-h2) cross product equation
            0.0,
            h0.velocity.z - h2.velocity.z,
            h2.velocity.y - h0.velocity.y,
            0.0,
            h2.position.z - h0.position.z,
            h0.position.y - h2.position.y,
            // Row 4: From y-component of (h0-h2) cross product equation
            h2.velocity.z - h0.velocity.z,
            0.0,
            h0.velocity.x - h2.velocity.x,
            h0.position.z - h2.position.z,
            0.0,
            h2.position.x - h0.position.x,
            // Row 5: From z-component of (h0-h2) cross product equation
            h0.velocity.y - h2.velocity.y,
            h2.velocity.x - h0.velocity.x,
            0.0,
            h2.position.y - h0.position.y,
            h0.position.x - h2.position.x,
            0.0,
        ];

        let a = Matrix6::from_row_slice(&data);

        // Construct the 6×1 right-hand side vector b.
        // Each entry is the constant term from expanding the cross product equations.
        // For hailstone i: position × velocity = (px*vy - py*vx, pz*vx - px*vz, px*vy - py*vx)
        // The RHS comes from: (p0 × v0) - (p1 × v1) for the first 3 equations,
        //                      (p0 × v0) - (p2 × v2) for the last 3 equations.

        let vector = Vector6::new(
            // x-component: (p0.y * v0.z - v0.y * p0.z) - (p1.y * v1.z - v1.y * p1.z)
            h0.position.y * h0.velocity.z
                - h0.velocity.y * h0.position.z
                - (h1.position.y * h1.velocity.z - h1.velocity.y * h1.position.z),
            // y-component: (p0.z * v0.x - v0.z * p0.x) - (p1.z * v1.x - v1.z * p1.x)
            h0.position.z * h0.velocity.x
                - h0.velocity.z * h0.position.x
                - (h1.position.z * h1.velocity.x - h1.velocity.z * h1.position.x),
            // z-component: (p0.x * v0.y - v0.x * p0.y) - (p1.x * v1.y - v1.x * p1.y)
            h0.position.x * h0.velocity.y
                - h0.velocity.x * h0.position.y
                - (h1.position.x * h1.velocity.y - h1.velocity.x * h1.position.y),
            // Same three components but for h0 vs h2
            h0.position.y * h0.velocity.z
                - h0.velocity.y * h0.position.z
                - (h2.position.y * h2.velocity.z - h2.velocity.y * h2.position.z),
            h0.position.z * h0.velocity.x
                - h0.velocity.z * h0.position.x
                - (h2.position.z * h2.velocity.x - h2.velocity.z * h2.position.x),
            h0.position.x * h0.velocity.y
                - h0.velocity.x * h0.position.y
                - (h2.position.x * h2.velocity.y - h2.velocity.x * h2.position.y),
        );

        // Solve the linear system A * x = b using LU decomposition
        // The solution x contains [rx, ry, rz, rvx, rvy, rvz]
        // We only need the sum of the rock's starting position: rx + ry + rz
        // Round to nearest integer (the rock's coordinates are whole numbers)
        a.lu()
            .solve(&vector)
            .expect("Linear system must have a solution")
            .iter()
            .take(3)
            .sum::<f64>()
            .ceil() as i64
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
    println!(
        "Rock coordinates sum -> {}",
        HailStone::position_throw_obliterate(&hailstones)
    )
}
