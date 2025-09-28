use std::collections::HashMap;

pub fn count_springs() {
    let arrangements = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| {
            let (spring, damaged_parts) = line
                .split_once(" ")
                .expect("Must contain a valid seperator.");

            // It's a common trick to add a '.' to the end,
            // As it guarantees any final group of '#' is properly terminated.
            let mut spring = [spring; 5].join("?").chars().collect::<Vec<_>>();
            spring.push('.');

            let damaged_parts = [damaged_parts; 5]
                .join(",")
                .split(",")
                .map(|val| val.parse::<usize>().expect("Must be a valid number."))
                .collect::<Vec<_>>();

            traverse(&spring, &damaged_parts, &mut HashMap::new(), 0, 0, 0)
        })
        .sum::<usize>();

    println!("\nTotal Arrangements -> {arrangements}");
}

fn traverse(
    spring: &[char],
    parts: &[usize],
    cache: &mut HashMap<(usize, usize, usize), usize>,
    spring_idx: usize,
    part_idx: usize,
    curr_len: usize,
) -> usize {
    if let Some(&val) = cache.get(&(spring_idx, part_idx, curr_len)) {
        return val;
    }
    if spring_idx == spring.len() {
        return if part_idx == parts.len() { 1 } else { 0 };
    }
    if part_idx == parts.len() {
        let res = if curr_len == 0 && !spring[spring_idx..].iter().any(|&c| c == '#') {
            1
        } else {
            0
        };
        cache.insert((spring_idx, part_idx, curr_len), res);
        return res;
    }

    let mut res = 0;
    if spring[spring_idx] == '.' || spring[spring_idx] == '?' {
        if curr_len == 0 {
            // Simply move to the next char.
            res += traverse(spring, parts, cache, spring_idx + 1, part_idx, curr_len);
        } else {
            // See if the curr_len matches the curr part.
            // If they do match, increase the part_idx else prune the branch.
            if curr_len == parts[part_idx] {
                res += traverse(spring, parts, cache, spring_idx + 1, part_idx + 1, 0);
            } else if spring[spring_idx] == '.' {
                // We have a chance to early prune it.
                return 0;
            }
        }
    }
    if spring[spring_idx] == '#' || spring[spring_idx] == '?' {
        res += traverse(spring, parts, cache, spring_idx + 1, part_idx, curr_len + 1);
    }

    cache.insert((spring_idx, part_idx, curr_len), res);
    return res;
}
