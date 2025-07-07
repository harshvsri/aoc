use crate::{lcm, read_file};
use std::collections::HashMap;

fn traverse_map(
    instructions: &Vec<char>,
    map: &HashMap<String, (String, String)>,
    mut current: String,
) -> usize {
    let mut steps = 0;

    loop {
        if current.ends_with('Z') {
            break;
        }

        let (l_pos, r_pos) = map
            .get(&current)
            .expect("Cant go anywhere from here, check your path.");

        let dir = instructions[steps % instructions.len()];
        match dir {
            'L' => current = l_pos.to_string(),
            'R' => current = r_pos.to_string(),
            _ => panic!("Invalid diraction."),
        }
        steps += 1;
    }

    steps
}

pub fn get_steps() {
    let content = read_file();

    let (instructions, map) = content
        .split_once("\n\n")
        .expect("Must contain a valid seperator.");

    let instructions = instructions.chars().collect::<Vec<_>>();

    let map = map
        .lines()
        .map(|line| {
            let (key, value) = line
                .split_once(" = ")
                .expect("input.txt must be present in the root of the directory.");

            let (l_val, r_val) = value
                .split_once(", ")
                .expect("input.txt must be present in the root of the directory.");

            let (l_val, r_val) = (l_val.replace("(", ""), r_val.replace(")", ""));
            (key.to_string(), (l_val, r_val))
        })
        .collect::<HashMap<String, (String, String)>>();

    let points = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| key.to_string())
        .collect::<Vec<_>>();

    let mut steps = Vec::new();
    for point in points {
        let curr_step = traverse_map(&instructions, &map, point.to_string()) as u64;
        steps.push(curr_step);
        println!("Steps took by {}: {}", point.to_string(), curr_step);
    }

    let total_steps = steps.iter().cloned().reduce(lcm).unwrap();
    println!("Total Steps: {}", total_steps);
}
