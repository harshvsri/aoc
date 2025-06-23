pub fn get_total_load() {
    let mut map = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    tilt_north(&mut map);

    let load = map
        .iter()
        .enumerate()
        .map(|(index, row)| row.iter().filter(|item| **item == 'O').count() * (map.len() - index))
        .sum::<usize>();

    println!("Total Load -> {:?}", load);
}

fn tilt_north(map: &mut Vec<Vec<char>>) {
    for row in 1..map.len() {
        for col in 0..map[0].len() {
            let (mut curr_row, mut prev_row) = (row, row - 1);

            while map[curr_row][col] == 'O' && map[prev_row][col] == '.' {
                map[prev_row][col] = 'O';
                map[curr_row][col] = '.';

                if prev_row > 0 {
                    curr_row = prev_row;
                    prev_row = prev_row - 1;
                } else {
                    break;
                }
            }
        }
    }
}
