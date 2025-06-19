fn get_next_record(values: &[i32]) -> i32 {
    let mut res = values.first().expect("Must not be empty.").clone();

    if values.iter().all(|val| *val == 0) {
        return res;
    }

    let diff_values = values
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    res -= get_next_record(&diff_values);
    return res;
}

pub fn get_extrapolated_value() {
    let reports = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().expect("Must be a valid number."))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let extrapolated_value = reports
        .iter()
        .map(|report| get_next_record(report))
        .sum::<i32>();
    println!("Extrapolated Value: {}", extrapolated_value);
}
