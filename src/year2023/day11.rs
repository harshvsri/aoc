pub fn get_distances() {
    let universe = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (empty_rows, empty_cols, galaxies) = analyze_universe(&universe);
    const EXPANSION_FACTOR: usize = 1000_000;

    let mut total_distance = 0;
    for galaxy_a_index in 0..galaxies.len() {
        for galaxy_b_index in galaxy_a_index + 1..galaxies.len() {
            let (x1, y1) = &galaxies[galaxy_a_index];
            let (x2, y2) = &galaxies[galaxy_b_index];

            for row in &empty_rows {
                if (row > x1 && row < x2) || (row > x2 && row < x1) {
                    // Row lies between the points we need to account to the expansion
                    total_distance += EXPANSION_FACTOR - 1;
                }
            }

            for col in &empty_cols {
                if (col > y1 && col < y2) || (col > y2 && col < y1) {
                    // Col lies between the points we need to account to the expansion
                    total_distance += EXPANSION_FACTOR - 1;
                }
            }

            total_distance += x2.abs_diff(*x1) + y2.abs_diff(*y1);
        }
    }
    println!("Total Distance: {}", total_distance);
}

fn analyze_universe(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>, Vec<(usize, usize)>) {
    let mut empty_rows = vec![true; universe.len()];
    let mut empty_cols = vec![true; universe[0].len()];
    let mut galaxis = vec![];

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                empty_rows[row] = false;
                empty_cols[col] = false;
                galaxis.push((row, col));
            }
        }
    }

    let empty_rows = empty_rows
        .iter()
        .enumerate()
        .filter(|(_, res)| **res)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let empty_cols = empty_cols
        .iter()
        .enumerate()
        .filter(|(_, res)| **res)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    (empty_rows, empty_cols, galaxis)
}
