use std::{collections::HashMap, fs, i32, iter::zip};

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let content = fs::read_to_string("./input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in content.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();

        let left_item = items
            .get(0)
            .expect("Valid number should be present.")
            .parse::<i32>()
            .expect("Expected a valid number.");
        left.push(left_item);

        let right_item = items
            .get(1)
            .expect("Valid number should be present.")
            .parse::<i32>()
            .expect("Expected a valid number.");
        right.push(right_item);
    }

    left.sort();
    right.sort();
    (left, right)
}

pub fn solve() {
    let (left, right) = parse_input();

    let (mut part_one_res, mut part_two_res) = (0, 0);

    // Solution for part 1.
    for (l_val, r_val) in zip(&left, &right) {
        part_one_res += (l_val - r_val).abs();
    }

    // Solution for part 2.
    let mut freq_table: HashMap<i32, i32> = HashMap::new();
    for num in &right {
        freq_table
            .entry(*num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for num in &left {
        let freq = match freq_table.get(num) {
            Some(val) => *val,
            None => 0,
        };
        part_two_res += num * freq;
    }

    println!("{}, {}", part_one_res, part_two_res)
}
