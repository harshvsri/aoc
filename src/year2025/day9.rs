pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("File should be present in the root directory.");

    let (mut min_x, mut max_x) = (u64::MAX, u64::MIN);
    let (mut min_y, mut max_y) = (u64::MAX, u64::MIN);
    let tiles = data
        .lines()
        .map(|l| {
            let (y, x) = l.split_once(",").expect("Must contain a valid delemeter.");
            let (x, y) = (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap());
            (min_x, max_x) = (min_x.min(x), max_x.max(x));
            (min_y, max_y) = (min_y.min(y), max_y.max(y));
            (x, y)
        })
        .collect::<Vec<_>>();
    println!("X: [{min_x},{max_x}] Y: [{min_y},{max_y}]");
    println!("Dimention: {} X {}", max_x - min_x + 1, max_y - min_y + 1);

    fn area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
        (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
    }

    let mut max_area = 0;
    for &p1 in &tiles {
        for &p2 in &tiles {
            if p1 != p2 {
                max_area = max_area.max(area(p1, p2));
            }
        }
    }
    println!("Max area: {max_area}");
}

// For PART2 we need to first make a polygon and sort all the vertics so that i get all the edges
// along them, and then i need to find every pair of red tiles and see if any point lies outdide
// the polygon and this can be done via RAY CAST ALGORITHM.
