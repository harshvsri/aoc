use std::fs;

pub fn _parse_input() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut safe_count = 0;

    for line in content.lines() {
        let items = line
            .split_whitespace()
            .map(|val| val.parse::<i32>().expect("Should be a valid number."))
            .collect::<Vec<i32>>();

        if _is_ordered(&items) && _is_safe(&items) || _can_handle_one_fault(&items) {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}

// Checks for two adjacent items differ by at least one and at most three.
fn _is_safe(items: &[i32]) -> bool {
    for index in 1..items.len() {
        let diff = (items[index] - items[index - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

// Checks for either all increasing or all decreasing.
fn _is_ordered(items: &[i32]) -> bool {
    if items.len() < 2 {
        return true;
    }

    let sign = items[1] < items[0];

    for idx in 1..items.len() {
        let curr_sign = items[idx] < items[idx - 1];
        if items[idx] == items[idx - 1] || sign != curr_sign {
            return false;
        }
    }

    return true;
}

// Checks if removing one level makes the report safe and ordered.
fn _can_handle_one_fault(items: &[i32]) -> bool {
    for idx in 0..items.len() {
        let filtered = items
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != idx)
            .map(|(_, val)| *val)
            .collect::<Vec<i32>>();

        if _is_safe(&filtered) && _is_ordered(&filtered) {
            return true;
        }
    }

    return false;
}
