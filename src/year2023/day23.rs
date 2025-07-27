use std::collections::HashSet;

enum Dir {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
impl Dir {
    fn to_coords(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }

    fn get_dirs(ch: char) -> Vec<Dir> {
        match ch {
            '.' => vec![Dir::North, Dir::East, Dir::South, Dir::West],
            '^' => vec![Dir::North],
            '>' => vec![Dir::East],
            'v' => vec![Dir::South],
            '<' => vec![Dir::West],
            _ => panic!("Invalid character found on map."),
        }
    }
}

const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

fn hike((x, y): (isize, isize), map: &Vec<Vec<char>>, set: &mut HashSet<(isize, isize)>) -> usize {
    if x == map.len() as isize - 1 && y == map[0].len() as isize - 2 {
        return 1;
    }

    if set.contains(&(x, y)) {
        return 0;
    }
    set.insert((x, y));

    let mut max_len = 0;
    for dir in DIRS {
        let (dx, dy) = dir.to_coords();
        let (x, y) = (x + dx, y + dy);

        if x >= 0 && x < map.len() as isize && y >= 0 && y < map[0].len() as isize {
            if map[x as usize][y as usize] != '#' {
                max_len = max_len.max(hike((x, y), map, set));
            }
        }
    }

    set.remove(&(x, y));

    if max_len != 0 {
        max_len += 1;
    }
    return max_len;
}

pub fn longest_hike() {
    let map = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_len = hike((0, 1), &map, &mut HashSet::new()) - 1;
    println!("Longest Hike -> {max_len} steps.");
}
