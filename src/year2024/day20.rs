use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const CHEAT_TIME: isize = 20;

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

fn time_to_finish(grid: &Vec<Vec<char>>, valid_positions: &[(isize, isize)]) {
    let pos_to_time: HashMap<(isize, isize), usize> = valid_positions
        .iter()
        .enumerate()
        .map(|(time, &pos)| (pos, time))
        .collect();
    let mut map = HashMap::new();

    for (start_time, &(x, y)) in valid_positions.iter().enumerate() {
        for cheat_time in 1..=CHEAT_TIME {
            for time in 0..=cheat_time {
                let (dx, dy) = (time, cheat_time - time);

                let deltas = if dx == 0 {
                    vec![(0, dy), (0, -dy)]
                } else if dy == 0 {
                    vec![(dx, 0), (-dx, 0)]
                } else {
                    vec![(dx, dy), (dx, -dy), (-dx, dy), (-dx, -dy)]
                };

                for (dx, dy) in deltas {
                    let (nx, ny) = (x + dx, y + dy);

                    if nx < 0 || nx == grid.len() as isize || ny < 0 || ny == grid[0].len() as isize
                    {
                        continue;
                    }

                    if let Some(&original_time) = pos_to_time.get(&(nx, ny)) {
                        let cheat_time = start_time + cheat_time as usize;
                        if cheat_time < original_time {
                            let time_saved = original_time - cheat_time;
                            *map.entry(time_saved).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    println!(
        "Number of cheats saving atleast 100 ps: {:?}",
        map.iter()
            .filter(|(k, _)| **k >= 100)
            .map(|(_, v)| v)
            .sum::<usize>()
    );
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
    println!("Valid positions: {}", valid_positions.len());
    time_to_finish(&grid, &valid_positions);
}
