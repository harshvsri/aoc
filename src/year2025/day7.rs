use std::collections::{HashMap, HashSet};

type Ray = (isize, isize);

pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let grid = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (
        0isize,
        grid[0]
            .iter()
            .enumerate()
            .find(|x| x.1 == &'S')
            .expect("Must contain a starting point.")
            .0 as isize,
    );
    println!("Total Splits: {}", split_count(start, &grid));
    println!(
        "Possible Rays: {}",
        ray_trace(start, &grid, &mut HashMap::new())
    );
}

fn ray_trace(ray: Ray, grid: &Vec<Vec<char>>, cache: &mut HashMap<Ray, u64>) -> u64 {
    let (x, y) = ray;
    let (nx, ny) = (x + 1, y);

    if nx == grid.len() as isize {
        return 1;
    }
    if let Some(&val) = cache.get(&(nx, ny)) {
        return val;
    }
    let mut count = 0;
    match grid[nx as usize][ny as usize] {
        '.' => count += ray_trace((nx, ny), grid, cache),
        '^' => {
            if y - 1 >= 0 {
                count += ray_trace((nx, ny - 1), grid, cache)
            }
            if y + 1 < grid[0].len() as isize {
                count += ray_trace((nx, ny + 1), grid, cache)
            }
        }
        _ => unreachable!(),
    }
    cache.insert((x, y), count);
    count
}

pub fn split_count(ray: Ray, grid: &Vec<Vec<char>>) -> u32 {
    let mut all_rays = HashSet::new();
    let mut rays = HashSet::from([ray]);
    let mut new_rays = HashSet::new();
    let mut splits = 0;

    for _ in 0..(grid.len() - 1) {
        for &ray in &rays {
            let (x, y) = ray;
            let (nx, ny) = (x + 1, y);

            match grid[nx as usize][ny as usize] {
                '.' => {
                    new_rays.insert((nx, ny));
                }
                '^' => {
                    if y - 1 >= 0 {
                        new_rays.insert((nx, ny - 1));
                    }
                    if y + 1 < grid[0].len() as isize {
                        new_rays.insert((nx, ny + 1));
                    }
                    splits += 1;
                }
                _ => unreachable!(),
            }
        }
        rays.clear();
        rays.extend(new_rays.drain());

        // INFO: Debug purposes
        all_rays.extend(&rays);
        // println!("Rays[{}]: {:?}", rays.len(), rays);
    }

    // INFO: Debug purposes
    let mut grid_clone = grid.clone();
    for (x, y) in all_rays {
        grid_clone[x as usize][y as usize] = '┃';
    }
    for row in 0..grid_clone.len() {
        for col in 0..grid_clone[0].len() {
            print!(" {} ", grid_clone[row][col]);
        }
        println!()
    }
    splits
}
