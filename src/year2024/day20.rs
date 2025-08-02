use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn find_char(ch: char, grid: &Vec<Vec<char>>) -> Option<(isize, isize)> {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == ch {
                return Some((row as isize, col as isize));
            }
        }
    }
    None
}

fn time_to_finish(
    start_pos: (isize, isize),
    end_pos: (isize, isize),
    grid: &Vec<Vec<char>>,
    valid_positions: &[(isize, isize)],
) {
    let mut pq = BinaryHeap::from([(Reverse(0usize), start_pos, 2)]);
    let mut visited = HashSet::new();

    while !pq.is_empty() {
        let (time, (x, y), cheat_moves) = pq.pop().unwrap();
        if x == end_pos.0 && y == end_pos.1 {
            println!("Time taken: {} ps.", time.0);
        }

        if !visited.insert((x, y, cheat_moves)) {
            // We have seen this.
            // if cheat_moves == 0 {
            //     if let Ok(idx) = valid_positions.binary_search(&(x, y)) {
            //         let time_taken = time.0 + valid_positions.len() - idx;
            //         if time_taken < valid_positions.len() {
            //             println!("Time taken: {} ps.", time_taken);
            //         }
            //     }
            // }
            continue;
        }

        for (dx, dy) in DIRS {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx == grid.len() as isize || ny < 0 || ny == grid[0].len() as isize {
                continue;
            }

            match cheat_moves {
                2 => {
                    pq.push((Reverse(time.0 + 1), (nx, ny), cheat_moves - 1));

                    if grid[nx as usize][ny as usize] != '#' {
                        pq.push((Reverse(time.0 + 1), (nx, ny), cheat_moves));
                    }
                }
                1 => {
                    pq.push((Reverse(time.0 + 1), (nx, ny), cheat_moves - 1));
                }
                0 => {
                    // Maybe we should spawn a new isolated search so that it would not be affected
                    // by the upcoming results.
                    // Since hashmap con have this state which will stop it from getting explored.
                    // Or i can do something so that hasmap will not stop them like say i have
                    // (x,y,0) now i could have been here from anywhere but just because the
                    // previous one explored it will stop curr from exploring.
                    // So either HACK around hashset or spawn it differently.
                    if grid[nx as usize][ny as usize] != '#' {
                        pq.push((Reverse(time.0 + 1), (nx, ny), cheat_moves));
                    }
                    // if let Ok(idx) = valid_positions.binary_search(&(nx, ny)) {
                    //     println!("Time taken: {} ps.", time.0 + valid_positions.len() - idx);
                    // }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}

fn get_valid_positions(
    start_pos: (isize, isize),
    end_pos: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> Vec<(isize, isize)> {
    let mut pq = BinaryHeap::from([(Reverse(0usize), start_pos)]);
    let mut visited = HashSet::from([start_pos]);
    let mut predecessor_map = HashMap::new();

    while let Some((time, (x, y))) = pq.pop() {
        if x == end_pos.0 && y == end_pos.1 {
            break;
        }

        for (dx, dy) in DIRS {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx == grid.len() as isize || ny < 0 || ny == grid[0].len() as isize {
                continue;
            }
            if grid[nx as usize][ny as usize] == '#' {
                continue;
            }

            if visited.insert((nx, ny)) {
                pq.push((Reverse(time.0 + 1), (nx, ny)));
                predecessor_map.insert((nx, ny), (x, y));
            }
        }
    }

    let mut path = Vec::new();
    let mut curr = end_pos;
    loop {
        path.push(curr);
        if let Some(&val) = predecessor_map.get(&curr) {
            curr = val;
        } else {
            break;
        }
    }
    path.reverse();
    path
}

pub fn foo() {
    let grid = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_pos, end_pos) = (
        find_char('S', &grid).unwrap(),
        find_char('E', &grid).unwrap(),
    );

    let valid_positions = get_valid_positions(start_pos, end_pos, &grid);
    println!("Valid Positions: {:?}", valid_positions.len());
    time_to_finish(start_pos, end_pos, &grid, &valid_positions);
}
