use crate::gridmap;
use std::{collections::HashMap, sync::OnceLock};

type Pos = (isize, isize);
const NUMPAD_GRID: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
#[rustfmt::skip]
const ARROW_GRID: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>']
];
const MAX_DEPTH: u8 = 25 + 1;

gridmap!(numpad_map, NUMPAD_MAP, NUMPAD_GRID);
gridmap!(arrow_map, ARROW_MAP, ARROW_GRID);

pub fn get_valid_paths(from: Pos, to: Pos, invalid: Pos) -> (Option<String>, Option<String>) {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);

    let horizontal = match dy.cmp(&0) {
        std::cmp::Ordering::Greater => &">".repeat(dy as usize),
        std::cmp::Ordering::Less => &"<".repeat(dy.abs() as usize),
        std::cmp::Ordering::Equal => "",
    };
    let vertical = match dx.cmp(&0) {
        std::cmp::Ordering::Greater => &"v".repeat(dx as usize),
        std::cmp::Ordering::Less => &"^".repeat(dx.abs() as usize),
        std::cmp::Ordering::Equal => "",
    };

    let verticalal_blocked = (from.0 + dx, from.1) == invalid;
    let horizontal_blocked = (from.0, from.1 + dy) == invalid;
    match (verticalal_blocked, horizontal_blocked) {
        (true, false) => (None, Some(String::from_iter([horizontal, vertical]))),
        (false, true) => (Some(String::from_iter([vertical, horizontal])), None),
        (false, false) => (
            Some(String::from_iter([vertical, horizontal])),
            Some(String::from_iter([horizontal, vertical])),
        ),
        (true, true) => panic!("There must be at least one valid path."),
    }
}

pub fn get_cost(
    from: char,
    to: char,
    depth: u8,
    cache: &mut HashMap<(char, char, u8), usize>,
) -> usize {
    if let Some(&cost) = cache.get(&(from, to, depth)) {
        return cost;
    }

    if depth == 0 {
        return 1;
    }

    let mut cost = usize::MAX;
    let map = if depth == MAX_DEPTH {
        numpad_map()
    } else {
        arrow_map()
    };

    let (left, right) = get_valid_paths(map[&from], map[&to], map[&' ']);
    if let Some(mut left) = left {
        left.insert(0, 'A');
        left.push('A');

        let mut left_cost = 0;
        for win in left.as_bytes().windows(2) {
            left_cost += get_cost(win[0] as char, win[1] as char, depth - 1, cache);
        }
        cost = cost.min(left_cost);
    }

    if let Some(mut right) = right {
        right.insert(0, 'A');
        right.push('A');

        let mut right_cost = 0;
        for win in right.as_bytes().windows(2) {
            right_cost += get_cost(win[0] as char, win[1] as char, depth - 1, cache);
        }
        cost = cost.min(right_cost);
    }

    cache.insert((from, to, depth), cost);
    return cost;
}

pub fn solve() {
    let mut sequences = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let res = sequences
        .iter_mut()
        .map(|sequence| {
            let numeric_val = sequence[..3].parse::<usize>().expect(&format!(
                "Must be a valid number but got {}.",
                &sequence[..3]
            ));
            sequence.insert(0, 'A');

            let mut cost = 0;
            let mut cache = HashMap::new();
            for win in sequence.as_bytes().windows(2) {
                cost += get_cost(win[0] as char, win[1] as char, MAX_DEPTH, &mut cache);
            }

            let res = numeric_val * cost;
            println!("{sequence} -> ({cost} * {numeric_val}) {}", res);
            res
        })
        .sum::<usize>();

    println!("Final res: {res}");
}
