pub fn count_springs() {
    let parts = vec!['.', '#'];

    let arrangements = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (spring, damaged_parts) = line
                .split_once(" ")
                .expect("Must contain a valid seperator.");

            // It's a common trick to add a '.' to the end,
            // As it guarantees any final group of '#' is properly terminated.
            let mut spring = spring.chars().collect::<Vec<_>>();
            spring.push('.');

            let damaged_parts = damaged_parts
                .split(",")
                .map(|val| val.parse::<u8>().expect("Must be a valid number."))
                .collect::<Vec<_>>();

            let count = traverse(0, &mut spring, &damaged_parts, &parts);
            println!("{: >3}. {:?} -> {count}", index, spring);
            count
        })
        .sum::<usize>();

    println!("\nTotal Arrangements -> {arrangements}");
}

fn traverse(index: usize, spring: &mut [char], damaged_parts: &[u8], parts: &[char]) -> usize {
    if index == spring.len() {
        if is_valid(spring, damaged_parts) {
            return 1;
        }
        return 0;
    }

    let mut res = 0;
    if spring[index] == '.' || spring[index] == '#' {
        res += traverse(index + 1, spring, damaged_parts, parts);
    } else {
        for &part in parts {
            spring[index] = part;
            res += traverse(index + 1, spring, damaged_parts, parts);
            spring[index] = '?';
        }
    }

    return res;
}

fn is_valid(spring: &[char], damaged_parts: &[u8]) -> bool {
    let found_groups = spring
        .split(|&c| c == '.')
        .filter(|group| !group.is_empty())
        .map(|group| group.len() as u8)
        .collect::<Vec<_>>();

    found_groups == damaged_parts
}
