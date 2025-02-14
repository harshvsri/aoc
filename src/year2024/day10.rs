use std::{collections::HashSet, fs};

pub fn get_trailheads() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .expect("Every character must be a valid number.")
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let starting_points = find_starting_points(&map);
    println!("Starting points: {:?}", starting_points);

    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut hiking_paths = 0;

    for (index, &(row, col)) in starting_points.iter().enumerate() {
        let paths = get_hiking_paths((row as i32, col as i32), &map, &dirs, &mut visited);
        println!("{}. Hiking paths from ({},{}): {}", index, row, col, paths);
        hiking_paths += paths;
    }

    println!("Total hiking paths: {}", hiking_paths);
}

fn find_starting_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut start_pos = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                start_pos.push((row, col));
            }
        }
    }
    start_pos
}

fn get_hiking_paths(
    pos: (i32, i32),
    map: &Vec<Vec<u32>>,
    dirs: &Vec<(i32, i32)>,
    visited: &mut HashSet<(i32, i32)>,
) -> u32 {
    let curr_val = map[pos.0 as usize][pos.1 as usize];
    if visited.len() == 9 {
        if curr_val == 9 {
            return 1;
        }
        return 0;
    }

    visited.insert(pos);

    let mut count = 0;
    for (dx, dy) in dirs {
        let next_pos = (pos.0 + dx, pos.1 + dy);

        if is_valid(map, next_pos)
            && !visited.contains(&next_pos)
            && map[next_pos.0 as usize][next_pos.1 as usize] == curr_val + 1
        {
            count += get_hiking_paths(next_pos, map, dirs, visited)
        }
    }

    visited.remove(&pos);
    count
}

fn is_valid(map: &Vec<Vec<u32>>, (row, col): (i32, i32)) -> bool {
    row >= 0 && row < map.len() as i32 && col >= 0 && col < map[0].len() as i32
}
