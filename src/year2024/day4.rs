use std::fs;

pub fn _find_xmas() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let char_grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut two_mas_count = 0;

    for row in 0..char_grid.len() {
        for col in 0..char_grid[0].len() {
            if char_grid[row][col] == 'A' && _is_two_mas(&char_grid, row, col) {
                two_mas_count += 1;
            }
        }
    }

    println!("Total TWO MAS patterns found: {}", two_mas_count);
}

fn _get_xmas_count(char_grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let max_row = char_grid.len();
    let max_col = char_grid[0].len();
    let mut count = 0;

    let directions: [(i8, i8); 8] = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Diagonal Up-Left
        (-1, 1),  // Diagonal Up-Right
        (1, -1),  // Diagonal Down-Left
        (1, 1),   // Diagonal Down-Right
    ];
    let word = ['X', 'M', 'A', 'S'];

    for &(dx, dy) in &directions {
        let (mut r, mut c) = (row as isize, col as isize);
        let mut matches = true;

        for idx in 1..word.len() {
            r += dx as isize;
            c += dy as isize;

            // Check bounds
            if r < 0 || r >= max_row as isize || c < 0 || c >= max_col as isize {
                matches = false;
                break;
            }

            // Check if the character matches
            if char_grid[r as usize][c as usize] != word[idx] {
                matches = false;
                break;
            }
        }

        // If all characters matched, increment the count
        if matches {
            count += 1;
        }
    }

    return count;
}

fn _is_two_mas(char_grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let max_row = char_grid.len();
    let max_col = char_grid[0].len();

    let directions = [
        (-1, -1), // Diagonal Up-Left
        (-1, 1),  // Diagonal Up-Right
        (1, -1),  // Diagonal Down-Left
        (1, 1),   // Diagonal Down-Right
    ];

    let words = [
        ['M', 'M', 'S', 'S'],
        ['S', 'M', 'S', 'M'],
        ['M', 'S', 'M', 'S'],
        ['S', 'S', 'M', 'M'],
    ];

    for word in &words {
        let mut matches = true;

        for (idx, &(dx, dy)) in directions.iter().enumerate() {
            let r = row as isize + dx;
            let c = col as isize + dy;

            // Check bounds
            if r < 0 || r >= max_row as isize || c < 0 || c >= max_col as isize {
                matches = false;
                break;
            }

            // Check if the character matches
            if char_grid[r as usize][c as usize] != word[idx] {
                matches = false;
                break;
            }
        }

        if matches {
            return true;
        }
    }

    return false;
}
