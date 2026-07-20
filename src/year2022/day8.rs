pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let mut count = 0;
    let map = parse_map(&data);
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let height = map[x][y];

            for &(dx, dy) in &dirs {
                let (mut nx, mut ny) = (x as isize + dx, y as isize + dy);

                let mut visible = true;
                while nx >= 0 && nx < map.len() as isize && ny >= 0 && ny < map[0].len() as isize {
                    if map[nx as usize][ny as usize] >= height {
                        visible = false;
                        break;
                    }
                    nx += dx;
                    ny += dy;
                }
                if visible {
                    count += 1;
                    break;
                }
                // Else we need to continue looking into other directions.
            }
        }
    }
    println!("Visible trees: {count}");
}

fn parse_map(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
