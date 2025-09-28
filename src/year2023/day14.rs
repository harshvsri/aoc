const BILLION: usize = 1000000000;

pub fn get_total_load() {
    let mut state = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map = std::collections::HashMap::new();
    let mut counter = 0;
    let (start, end) = loop {
        if let Some(value) = map.get(&state) {
            println!("Found a cycle.");
            break (*value, counter);
        }
        map.insert(state.clone(), counter);
        cycle_state(&mut state);
        counter += 1;
    };

    let res = (BILLION - start) % (end - start);
    for (k, v) in &map {
        if *v == start + res {
            let load = k
                .iter()
                .enumerate()
                .map(|(index, row)| {
                    row.iter().filter(|item| **item == 'O').count() * (state.len() - index)
                })
                .sum::<usize>();

            println!("Total Load -> {:?}", load);
        }
    }
}

fn tilt(state: &mut Vec<Vec<char>>) {
    for row in 1..state.len() {
        for col in 0..state[0].len() {
            let (mut curr_row, mut prev_row) = (row, row - 1);

            while state[curr_row][col] == 'O' && state[prev_row][col] == '.' {
                state[prev_row][col] = 'O';
                state[curr_row][col] = '.';

                if prev_row > 0 {
                    curr_row = prev_row;
                    prev_row = prev_row - 1;
                } else {
                    break;
                }
            }
        }
    }
}

fn transpose(state: &mut Vec<Vec<char>>) {
    let len = state.len();

    // Transpose the matrix
    for i in 0..len {
        for j in 0..i {
            let tmp = state[i][j];
            state[i][j] = state[j][i];
            state[j][i] = tmp;
        }
    }

    // Reverse each row
    for row in state.iter_mut() {
        row.reverse();
    }
}

fn cycle_state(state: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        tilt(state);
        transpose(state);
    }
}
