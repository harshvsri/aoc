use std::cmp::{max, min};

fn get_file_content() -> String {
    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
}

fn make_map(map_str: &str) -> Vec<(usize, usize, usize)> {
    let mut map = map_str
        .split_once(":\n")
        .expect("Must contain : as a valid seperator")
        .1
        .lines()
        .map(|line| {
            let parts = line
                .split_whitespace()
                .map(|value| value.parse::<usize>().expect("Must be a valid number."))
                .collect::<Vec<_>>();

            let [destination, source, range] = parts.try_into().expect("Expected exactly 3 parts");
            (source, destination, range)
        })
        .collect::<Vec<_>>();

    map.sort_by_key(|&(source, _, _)| source);
    map
}

fn get_min_location(ranges: &[(usize, usize)], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    let mut ranges = ranges.to_vec();
    for index in 0..maps.len() {
        ranges = transform(&ranges, &maps[index]);
    }

    ranges
        .iter()
        .map(|&(start, _)| start)
        .min()
        .expect("Ranges must not be empyt.")
}

fn transform(ranges: &[(usize, usize)], map: &[(usize, usize, usize)]) -> Vec<(usize, usize)> {
    let mut modified_ranges = Vec::new();
    for &(start, length) in ranges {
        let (mut last_overlap_end, end) = (start, start + length);

        for &(source, destination, range) in map {
            if last_overlap_end >= end {
                break;
            }

            let overlap_start = max(start, source);
            let overlap_end = min(start + length, source + range);

            if overlap_start < end && overlap_start > last_overlap_end {
                // This is untransformed section.
                modified_ranges.push((last_overlap_end, overlap_start - last_overlap_end));
            }

            if overlap_end > overlap_start {
                let overlap_range = overlap_end - overlap_start;
                modified_ranges.push((destination + (overlap_start - source), overlap_range));
                last_overlap_end = overlap_end;
            }
        }

        if last_overlap_end < start + length {
            modified_ranges.push((last_overlap_end, (start + length - last_overlap_end)));
        }
    }
    return modified_ranges;
}

pub fn get_minimum_location() {
    let content = get_file_content();
    let (seeds, maps) = content.split_once("\n\n").expect("Must contain seperator.");

    let seeds = seeds
        .split_once(": ")
        .expect("Must contain : as a valid seperator")
        .1
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().expect("Must be a valid number."))
        .collect::<Vec<_>>();

    let maps = maps
        .split("\n\n")
        .map(|map_str| make_map(map_str))
        .collect::<Vec<_>>();

    let min_location = seeds
        .chunks(2)
        .map(|chunk| {
            assert!(chunk.len() == 2, "Invalid chunk lenght found, must be 2.");

            let (initial_seed, range) = (chunk[0], chunk[1]);
            let location = get_min_location(&vec![(initial_seed, range)], &maps);
            println!("[{}, {}] -> {}", initial_seed, range, location);
            location
        })
        .min()
        .expect("Seeds must not be empyt.");
    println!("Minimum location -> {}", min_location);
}
