use std::fs;

pub fn get_tokens_to_win() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");
    let content = content.split("\n\n").collect::<Vec<_>>();

    let mut tokens = 0;
    for chunk in content {
        tokens += get_tokens(chunk);
    }
    println!("Min Tokens: {tokens}");
}

fn get_tokens(chunk: &str) -> i64 {
    let chunk = chunk
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (x, y) = line
                .split_once(", ")
                .expect("Must be a valid string with (,) as seperator");

            let delimiter = if index < 2 { "+" } else { "=" };

            let (_, x) = x
                .split_once(delimiter)
                .expect("Must be a string with a valid seperator");
            let x = x.parse::<i64>().expect("Must be a valid number");

            let (_, y) = y
                .split_once(delimiter)
                .expect("Must be a string with a valid seperator");
            let y = y.parse::<i64>().expect("Must be a valid number");

            (x, y)
        })
        .collect::<Vec<_>>();

    let offset: i64 = 10000000000000;
    let (a, d) = chunk[0];
    let (b, e) = chunk[1];
    let (c, f) = (chunk[2].0 + offset, chunk[2].1 + offset);

    match solve_linear_equations(a, b, c, d, e, f) {
        Some((x, y)) => 3 * x + y,
        None => 0,
    }
}

fn solve_linear_equations(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> Option<(i64, i64)> {
    let determinant = a * e - b * d;
    if determinant == 0 {
        return None;
    }

    let x = (c * e - b * f) as f64 / determinant as f64;
    let y = (a * f - c * d) as f64 / determinant as f64;

    if x - x.trunc() != 0.0 || y - y.trunc() != 0.0 {
        return None;
    }
    let (x, y) = (x as i64, y as i64);

    println!("We might have a solution ({x}, {y})");
    if x < 0 || y < 0 {
        None
    } else {
        Some((x, y))
    }
}
