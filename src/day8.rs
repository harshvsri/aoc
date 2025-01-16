use std::{collections::HashMap, fs};

pub fn get_antinodes() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '.' {
                map.entry(grid[row][col])
                    .or_insert_with(Vec::new)
                    .push((row, col));
            }
        }
    }

    let (row_max, col_max) = (grid.len(), grid[0].len());
    for (_, value) in &map {
        for point1 in value {
            for point2 in value {
                match get_antinode_pos(*point1, *point2, (row_max as i32, col_max as i32)) {
                    Some((x, y)) => grid[x][y] = '#',
                    None => {}
                }
            }
        }
    }

    // Convert the grid to a string
    let mut res_data = String::new();
    for row in &grid {
        let line = row
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join("");

        res_data.push_str(&line);
        res_data.push('\n');
    }

    let antinode_count = res_data
        .chars()
        .filter(|ch| *ch == '#')
        .map(|_| 1)
        .sum::<u32>();
    println!("Unique antinode Count: {}", antinode_count);

    fs::write("res.txt", res_data).expect("File write into [res.txt] should happen as expected.");
}

fn get_antinode_pos(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (row_max, col_max): (i32, i32),
) -> Option<(usize, usize)> {
    if x1 == x2 && y1 == y2 {
        return None;
    }
    let (dx, dy) = (x1 as i32 - x2 as i32, y1 as i32 - y2 as i32);
    let (x, y) = (x1 as i32 + dx, y1 as i32 + dy);

    if is_valid_pos((x, y), row_max, col_max) {
        return Some((x as usize, y as usize));
    }
    return None;
}

fn is_valid_pos((x, y): (i32, i32), row_max: i32, col_max: i32) -> bool {
    x >= 0 && x < row_max && y >= 0 && y < col_max
}
