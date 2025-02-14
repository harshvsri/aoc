use std::fs;

pub fn parse_data() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let lines = content
        .lines()
        .map(|line| {
            let (res, values) = line
                .split_once(": ")
                .expect("String should have a valid seperator.");

            let res = res
                .parse::<u64>()
                .expect(&format!("Result string should be a valid number. {}", res));

            let values = values
                .split(" ")
                .map(|value| {
                    value
                        .parse::<u64>()
                        .expect("String should be a parsable valid number.")
                })
                .collect::<Vec<_>>();

            (res, values)
        })
        .collect::<Vec<_>>();

    let ops = vec!["+", "*", "||"];

    let res = lines
        .iter()
        .filter(|(target, values)| can_reach_target(values, 1, values[0], *target, &ops))
        .map(|(target, _)| target)
        .sum::<u64>();
    println!("Total sum: {}", res);
}

fn can_reach_target(values: &[u64], idx: usize, curr_res: u64, target: u64, ops: &[&str]) -> bool {
    // Base Case
    if curr_res > target {
        return false;
    }
    if idx == values.len() {
        return curr_res == target;
    }

    for op in ops {
        let new_res = apply_operator(curr_res, op, values[idx]);
        if can_reach_target(values, idx + 1, new_res, target, ops) {
            return true;
        }
    }

    return false;
}

fn apply_operator(lhs: u64, op: &str, rhs: u64) -> u64 {
    match op {
        "+" => lhs + rhs,
        "*" => lhs * rhs,
        "||" => (lhs * (10_u64.pow(get_digit_count(rhs)))) + rhs,
        _ => panic!("Invalid operator"),
    }
}

fn get_digit_count(num: u64) -> u32 {
    if num == 0 {
        1
    } else {
        (num as f64).log10().floor() as u32 + 1
    }
}
