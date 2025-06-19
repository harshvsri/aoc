fn get_file_content() -> String {
    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
}

fn get_record_broke_count(time: usize, record: usize) -> usize {
    let mut record_broke = 0;
    for t in 0..=time {
        let distance_covered = t * (time - t);
        if distance_covered > record {
            record_broke += 1;
        }
    }
    record_broke
}

pub fn parse_data_and_solve() {
    let content = get_file_content();

    let records = content
        .lines()
        .map(|line| {
            line.split_once(":")
                .expect("Must contain a valid seperator.")
                .1
                .trim()
                .split_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .expect("Must be a valid number")
        })
        .collect::<Vec<_>>();

    let [time, distance] = records.try_into().expect("Must be in a valid format");
    println!("Res: {}", get_record_broke_count(time, distance));
}
