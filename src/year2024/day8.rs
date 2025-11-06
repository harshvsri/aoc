use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn get_antinodes() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '.' {
                map.entry(grid[row][col]).or_default().push((row, col));
            }
        }
    }
    println!("{:?}", map);

    let (row_max, col_max) = (grid.len(), grid[0].len());
    let mut unique_antinodes = HashSet::new();

    for freq in map.values() {
        for point1 in freq {
            for point2 in freq {
                antinodes(*point1, *point2, (row_max, col_max), &mut unique_antinodes)
            }
        }
    }

    println!("Total unique antinodes: {}", unique_antinodes.len(),);

    // NOTE: This is done for visualization purpose.
    //
    // for &(x, y) in &unique_antinodes {
    //     if grid[x as usize][y as usize] == '.' {
    //         grid[x as usize][y as usize] = '#';
    //     }
    // }
    //
    // let s = grid
    //     .iter()
    //     .map(|row| {
    //         row.iter()
    //             .map(|num| num.to_string())
    //             .collect::<Vec<String>>()
    //             .join("")
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // fs::write("res.txt", s).expect("File write into [res.txt] should happen as expected.");
}

fn antinodes(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (row_max, col_max): (usize, usize),
    unique_antinodes: &mut HashSet<(i32, i32)>,
) {
    if x1 == x2 && y1 == y2 {
        return;
    }
    let (dx, dy) = (x1 as i32 - x2 as i32, y1 as i32 - y2 as i32);

    let (mut x, mut y) = (x1 as i32, y1 as i32);
    loop {
        (x, y) = (x + dx, y + dy);
        if is_valid_pos((x, y), row_max as i32, col_max as i32) {
            unique_antinodes.insert((x, y));
        } else {
            break;
        }
    }

    let (mut x, mut y) = (x1 as i32, y1 as i32);
    loop {
        (x, y) = (x - dx, y - dy);
        if is_valid_pos((x, y), row_max as i32, col_max as i32) {
            unique_antinodes.insert((x, y));
        } else {
            break;
        }
    }
}

fn is_valid_pos((x, y): (i32, i32), row_max: i32, col_max: i32) -> bool {
    x >= 0 && x < row_max && y >= 0 && y < col_max
}

#[allow(unused)]
fn get_antinode_pos(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (row_max, col_max): (usize, usize),
) -> Option<(usize, usize)> {
    if x1 == x2 && y1 == y2 {
        return None;
    }
    let (dx, dy) = (x1 as i32 - x2 as i32, y1 as i32 - y2 as i32);
    let (x, y) = (x1 as i32 + dx, y1 as i32 + dy);

    if is_valid_pos((x, y), row_max as i32, col_max as i32) {
        return Some((x as usize, y as usize));
    }
    return None;
}
