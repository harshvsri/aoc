use std::collections::HashMap;

pub fn foo() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let (patterns, designs) = content
        .split_once("\n\n")
        .expect("Must contain a valid seperator.");

    let patterns = patterns.trim().split(", ").collect::<Vec<_>>();
    let designs = designs.trim().lines().collect::<Vec<_>>();

    println!(
        "Possible designs: {}",
        designs
            .iter()
            .map(|design| get_valid_combos(design, 0, &patterns, &mut HashMap::new()))
            .sum::<usize>()
    );
}

fn get_valid_combos(
    design: &str,
    index: usize,
    patterns: &[&str],
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if index >= design.len() {
        return if index == design.len() { 1 } else { 0 };
    }
    if cache.contains_key(&index) {
        return cache.get(&index).unwrap().clone();
    }

    let mut count = 0;
    for pattern in patterns {
        if design[index..].starts_with(pattern) {
            count += get_valid_combos(design, index + pattern.len(), patterns, cache);
        }
    }

    cache.insert(index, count);
    return count;
}
