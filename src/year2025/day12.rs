pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    match data.split("\n\n").collect::<Vec<_>>().as_slice() {
        [shapes @ .., regions] => {
            let shapes = shapes
                .iter()
                .map(|shape| {
                    shape
                        .lines()
                        .skip(1)
                        .map(|line| line.chars().filter(|&c| c == '#').count())
                        .sum::<usize>()
                })
                .collect::<Vec<_>>();

            let fit_regions = regions
                .lines()
                .map(|region| {
                    let (dimention, qunatities) = region.split_once(": ").unwrap();
                    let (x, y) = dimention.split_once("x").unwrap();
                    let grid_area = x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();

                    let pieces_area = qunatities
                        .split_whitespace()
                        .enumerate()
                        .map(|(i, q)| shapes[i] * q.parse::<usize>().unwrap())
                        .sum::<usize>();
                    (grid_area, pieces_area)
                })
                .filter(|(ga, pa)| ga >= pa)
                .count();
            println!("Fit regions: {fit_regions}");
        }
        _ => unreachable!(),
    }
}
