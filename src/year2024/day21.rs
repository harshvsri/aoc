use std::collections::HashMap;
type Pos = (isize, isize);
const NUMPAD_GRID: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
const ARROW_GRID: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

// This fn doesnt handle the invalid case ie position that can cause panic the robot.
// Maybe this fn should also accept the invalid position so that we can validate all the paths.
pub fn get_valid_paths(from: &Pos, to: &Pos, invalid: &Pos) -> [Option<String>; 2] {
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

    let verticalal_blocked = &(from.0 + dx, from.1) == invalid;
    let horizontal_blocked = &(from.0, from.1 + dy) == invalid;
    match (verticalal_blocked, horizontal_blocked) {
        (true, false) => [None, Some(String::from_iter([horizontal, vertical]))],
        (false, true) => [Some(String::from_iter([vertical, horizontal])), None],
        (false, false) => [
            Some(String::from_iter([vertical, horizontal])),
            Some(String::from_iter([horizontal, vertical])),
        ],
        (true, true) => panic!("There must be at least one valid path."),
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

        // We can have multiple path options and we need to consider every path since we have no
        // idea which might be the most efficient.
        for ch in self.chars() {
            if let Some(target_pos) = map.get(&ch) {
                // We must account for the invalid paths since that may exponentially shoot up the total possibilities.
                // Right now this returns a random path, just for testing purposes.
                let [_a, _b] = get_valid_paths(current_pos, target_pos, invalid_pos);
                result.push('A');
                current_pos = target_pos;
            }
        }

        dbg!(result)
    }
}

pub fn foo() {
    let sequences = std::fs::read_to_string("test.txt")
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
                .to_string()
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
