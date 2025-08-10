use std::collections::HashMap;

const NUMPAD_GRID: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

const ARROW_GRID: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

fn get_path(from: &(isize, isize), to: &(isize, isize), invalid_pos: &(isize, isize)) -> String {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);

    let horizontal = match dy.cmp(&0) {
        std::cmp::Ordering::Greater => ">".repeat(dy as usize),
        std::cmp::Ordering::Less => "<".repeat(dy.abs() as usize),
        std::cmp::Ordering::Equal => String::new(),
    };

    let vertical = match dx.cmp(&0) {
        std::cmp::Ordering::Greater => "v".repeat(dx as usize),
        std::cmp::Ordering::Less => "^".repeat(dx.abs() as usize),
        std::cmp::Ordering::Equal => String::new(),
    };

    if &(from.0 + dx, from.1) == invalid_pos {
        format!("{}{}A", horizontal, vertical)
    } else if &(from.0, from.1 + dy) == invalid_pos {
        format!("{}{}A", vertical, horizontal)
    } else {
        // Now we need to find out which one is more optimal.
        // format!("{}{}A", horizontal, vertical);
        // format!("{}{}A", vertical, horizontal);
        todo!()
    }
}

trait KeypadSequence {
    fn get_seq(&self, grid: &[[char; 3]]) -> String;
}

impl KeypadSequence for String {
    fn get_seq(&self, grid: &[[char; 3]]) -> String {
        let map = grid
            .iter()
            .enumerate()
            .flat_map(|(row, row_data)| {
                row_data
                    .iter()
                    .enumerate()
                    .map(move |(col, &ch)| (ch, (row as isize, col as isize)))
            })
            .collect::<HashMap<_, _>>();

        // Start from 'A' position
        let mut current_pos = map.get(&'A').unwrap();
        let invalid_pos = map.get(&' ').unwrap();
        let mut result = String::new();

        for ch in self.chars() {
            if let Some(target_pos) = map.get(&ch) {
                result.push_str(&get_path(current_pos, target_pos, invalid_pos));
                current_pos = target_pos;
            }
        }

        result
    }
}

pub fn foo() {
    let sequences = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let res = sequences
        .iter()
        .map(|sequence| {
            let numeric_val = sequence[..3].parse::<usize>().expect(&format!(
                "Must be a valid number but got {}.",
                &sequence[..3]
            ));

            let seq_len = sequence
                .clone()
                .get_seq(&NUMPAD_GRID)
                .get_seq(&ARROW_GRID)
                .get_seq(&ARROW_GRID)
                .len();

            let res = numeric_val * seq_len;
            println!("{sequence} -> ({seq_len} * {numeric_val}) {}", res);
            res
        })
        .sum::<usize>();

    println!("{res}");
}
