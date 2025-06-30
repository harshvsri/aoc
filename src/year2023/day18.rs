#[derive(PartialEq, Debug)]
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

    fn get_dir(dir: &str) -> Self {
        match dir {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => panic!("Invalid direction found."),
        }
    }
}

/// Calculates the total area of a polygon defined by a series of instructions,
/// including the boundary and interior points.
///
/// This problem is a classic application of computational geometry, solved by combining
/// two key mathematical theorems: the **Shoelace Formula** and **Pick's Theorem**.
/// This approach is highly efficient as it avoids creating a grid and "filling" it,
/// which would be infeasible given the large coordinates in the problem.
///
/// ### Method
///
/// 1.  **Boundary Calculation (`B`)**:
///     - First, we determine the vertices of the polygon by following the given path
///       instructions.
///     - The number of integer points on the boundary (`B`) is simply the perimeter of
///       the polygon. Since the path consists of only horizontal and vertical lines,
///       this is the sum of the lengths of all the steps.
///
/// 2.  **Area Calculation (`A`): The Shoelace Formula**:
///     - The Shoelace Formula (or Surveyor's Formula) calculates the area of a simple
///       polygon given the ordered coordinates of its vertices `(x₁, y₁), (x₂, y₂), ...`.
///     - The formula is: `A = 0.5 * |(x₁y₂ + x₂y₃ + ... + xₙy₁) - (y₁x₂ + y₂x₃ + ... + yₙx₁)|`
///     - We apply this formula to the list of vertices we generated to find the exact
///       area enclosed by the trench.
///
/// 3.  **Interior Points (`I`): Pick's Theorem**:
///     - Pick's Theorem provides a relationship between the area of a polygon and the
///       integer points on its boundary and in its interior.
///     - The theorem states: `A = I + (B / 2) - 1`
///       - `A`: Area of the polygon (from Shoelace).
///       - `I`: Number of integer points in the **interior**.
///       - `B`: Number of integer points on the **boundary**.
///
/// 4.  **Finding the Total**:
///     - The goal is to find the total number of points, which is `Total = I + B`.
///     - We can rearrange Pick's Theorem to solve for `I`: `I = A - (B / 2) + 1`.
///     - Substituting this into our total equation gives:
///       `Total = (A - (B / 2) + 1) + B`
///     - This simplifies to the final, elegant formula:
///       **`Total = A + (B / 2) + 1`**
///
/// By calculating the area `A` and the boundary points `B`, we can find the total
/// number of lava-filled tiles directly.
pub fn get_total_lava() {
    let steps = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .trim()
        .lines()
        .map(|line| {
            let info = line.split_whitespace().collect::<Vec<_>>();
            assert!(info.len() == 3);
            (
                Direction::get_dir(&info[2][7..8]),
                usize::from_str_radix(&info[2][2..7], 16).expect("Must be a valid number."),
            )
        })
        .collect::<Vec<_>>();

    let mut vertex_points = vec![(0, 0)];
    let mut perimeter = 0;
    let (mut x, mut y) = (0, 0);

    for (dir, value) in &steps {
        let (dx, dy) = dir.to_coords();
        (x, y) = (x + (dx * *value as isize), y + (dy * *value as isize));

        vertex_points.push((x, y));
        perimeter += value;
    }
    println!(
        "Vertex Points({})[{perimeter}]: {:?}",
        vertex_points.len(),
        vertex_points
    );
    let area = calculate_area(&vertex_points);
    println!("Total Points: {}", area + (perimeter / 2) + 1);
}

fn calculate_area(vertex_points: &Vec<(isize, isize)>) -> usize {
    let (mut sum1, mut sum2) = (0, 0);
    for index in 0..(vertex_points.len() - 1) {
        let (x1, y1) = vertex_points[index];
        let (x2, y2) = vertex_points[index + 1];
        sum1 += x1 * y2;
        sum2 += y1 * x2;
    }
    (sum1.abs_diff(sum2)) / 2
}
