fn get_file_content() -> String {
    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
}

fn make_map(map_str: &str) -> Vec<(usize, usize, usize)> {
    map_str
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
        .collect::<Vec<_>>()
}

fn get_location(seed: usize, index: usize, maps: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    if index == maps.len() {
        return seed;
    }

    let seed = get_value_from_map(seed, &maps[index]);
    get_location(seed, index + 1, maps)
}

fn get_value_from_map(seed: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for &(source, destination, range) in map {
        if source <= seed && seed < source + range {
            // This contains the value for seed.
            return destination + (seed - source);
        }
    }
    return seed;
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

            let mut location = usize::MAX;
            for seed in initial_seed..initial_seed + range {
                location = location.min(get_location(seed, 0, &maps));
            }
            location
        })
        .min()
        .expect("Seeds must not be empyt.");
    println!("Minimum location -> {}", min_location);
}
