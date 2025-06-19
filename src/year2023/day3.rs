use std::{collections::HashSet, fmt::Display, fs, hash::Hash};

pub fn get_part_numbers() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let digits = "0123456789".chars().collect::<HashSet<char>>();
    let mut symbols = HashSet::new();
    let mut numbers = Vec::new();

    let mut value = 0;
    let mut start: Option<(usize, usize)> = None;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if digits.contains(&grid[row][col]) {
                // We found a digit.
                if value == 0 {
                    start = Some((row, col));
                }
                value *= 10;
                value += grid[row][col].to_digit(10).expect("Must be a valid digit.") as usize;
            } else {
                // We found a symbol or a dot.
                if grid[row][col] != '.' {
                    symbols.insert((row as isize, col as isize));
                }

                if value != 0 {
                    numbers.push(Number {
                        value,
                        start: start.expect("Must be intitialized, should not be None"),
                        end: (row, col - 1),
                    });
                }
                value = 0;
            }
        }

        // Handles digits occuring at the end.
        if value != 0 {
            numbers.push(Number {
                value,
                start: start.expect("Must be intitialized, should not be None"),
                end: (row, grid[0].len() - 1),
            });

            value = 0;
        }
    }

    let _res = numbers
        .iter()
        .filter(|num| num.has_symbol(&symbols))
        .map(|num| num.value)
        .sum::<usize>();

    get_valid_symbols(&symbols, &numbers);
}

fn get_valid_symbols(symbols: &HashSet<(isize, isize)>, numbers: &Vec<Number>) {
    let dirs = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut total_gear_ratio = 0;

    for &(x, y) in symbols {
        let mut num_set: HashSet<Number> = HashSet::new();

        for (dx, dy) in dirs {
            let (nx, ny) = (x + dx, y + dy);

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                num_set.extend(numbers.iter().filter(|num| num.contains_pos((nx, ny))))
            }
        }

        if num_set.len() == 2 {
            let gear_ratio = num_set.iter().map(|num| num.value).product::<usize>();
            total_gear_ratio += gear_ratio;

            println!("Gear ratio for {:?} -> {}.", (x, y), gear_ratio);
        }
    }
    println!("Total Gear Ratio {}.", total_gear_ratio);
}

// Represents a number in the grid with its start and end positions.
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Number {
    value: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>5} [{:?} -> {:?}]", self.value, self.start, self.end)
    }
}

impl Number {
    fn has_symbol(self, symbols: &HashSet<(isize, isize)>) -> bool {
        let row = self.start.0 as isize;
        let (col_start, col_end) = (self.start.1 as isize, self.end.1 as isize);

        if symbols.contains(&(row, col_start - 1)) || symbols.contains(&(row, col_end + 1)) {
            return true;
        }

        for col in col_start - 1..=col_end + 1 {
            if symbols.contains(&(row - 1, col)) || symbols.contains(&(row + 1, col)) {
                return true;
            }
        }
        return false;
    }

    fn contains_pos(self, (x, y): (usize, usize)) -> bool {
        let row = self.start.0;
        let (col_start, col_end) = (self.start.1, self.end.1);

        x == row && y >= col_start && y <= col_end
    }
}
