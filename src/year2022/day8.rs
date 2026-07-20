pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let mut max_view_value = 1;
    let map = parse_map(&data);
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let height = map[x][y];

            let mut view_value = 1;
            for &(dx, dy) in &dirs {
                let (mut nx, mut ny) = (x as isize + dx, y as isize + dy);

                let mut tree_count = 0;
                while nx >= 0 && nx < map.len() as isize && ny >= 0 && ny < map[0].len() as isize {
                    tree_count += 1;
                    if map[nx as usize][ny as usize] >= height {
                        break;
                    }
                    nx += dx;
                    ny += dy;
                }
                view_value *= tree_count;
                if view_value == 0 {
                    break;
                }
            }
            max_view_value = max_view_value.max(view_value);
        }
    }
    println!("Max View: {max_view_value}");
}

fn parse_map(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
