use std::{collections::HashSet, fs};

pub fn get_all_trailheads_score() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .expect("Every character must be a valid number.") as u8
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let trailheads = find_points(&map, 0);
    let trailends = find_points(&map, 9);
    println!("Starting points: {:?}", trailheads);
    println!("Ending points: {:?}", trailends);

    let dirs: Vec<(i8, i8)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::new();
    let mut visited_trailends = HashSet::new();

    let mut total_score = 0;

    for (index, &(row, col)) in trailheads.iter().enumerate() {
        visited_trailends.clear();

        let trailhead_score = get_trailheads_score(
            (row, col),
            &map,
            &dirs,
            &mut visited,
            &mut visited_trailends,
        );
        println!("{}. ({},{}) -> {}", index + 1, row, col, trailhead_score);
        total_score += trailhead_score;
    }

    println!("Total hiking paths: {}", total_score);
}

fn find_points(map: &Vec<Vec<u8>>, value: u8) -> Vec<(usize, usize)> {
    let mut start_pos = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == value {
                start_pos.push((row, col));
            }
        }
    }
    start_pos
}

fn get_trailheads_score(
    pos: (usize, usize),
    map: &Vec<Vec<u8>>,
    dirs: &Vec<(i8, i8)>,
    visited: &mut HashSet<(usize, usize)>,
    visited_trailends: &mut HashSet<(usize, usize)>,
) -> u32 {
    let curr_val = map[pos.0][pos.1];

    if visited.len() == 9 && curr_val == 9 && !visited_trailends.contains(&pos) {
        visited_trailends.insert(pos);
        return 1;
    }

    visited.insert(pos);
    let mut count = 0;
    for &(dx, dy) in dirs {
        let next_pos = (pos.0 as isize + dx as isize, pos.1 as isize + dy as isize);
        if is_valid(map, next_pos) {
            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            if !visited.contains(&next_pos) && map[next_pos.0][next_pos.1] == curr_val + 1 {
                count += get_trailheads_score(next_pos, map, dirs, visited, visited_trailends)
            }
        }
    }

    visited.remove(&pos);
    count
}

fn is_valid(map: &Vec<Vec<u8>>, (row, col): (isize, isize)) -> bool {
    row >= 0 && row < map.len() as isize && col >= 0 && col < map[0].len() as isize
}
