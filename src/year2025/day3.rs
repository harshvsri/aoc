pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let power_banks = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total_joltage = 0u32;
    for pb in power_banks {
        let mut max_joltage = 0;
        for i in 0..pb.len() {
            for j in i + 1..pb.len() {
                let joltage = (pb[i] as u8 - b'0') * 10 + (pb[j] as u8 - b'0');

                max_joltage = max_joltage.max(joltage);
            }
        }
        total_joltage += max_joltage as u32;
    }
    println!("Total_joltage -> {total_joltage}");

    // Solution for Part 2
    // println!(
    //     "Total joltage: {}",
    //     data.lines().map(|l| get_max_num(l, 12)).sum::<u64>()
    // );
}

pub fn get_max_num(s: &str, num_len: usize) -> u64 {
    let mut start = 0;
    (1..=num_len)
        .map(|i| {
            let end = s.len() - (num_len - i);
            let (index, max_digit) = s[start..end]
                .chars()
                .enumerate()
                .reduce(|a, b| if b.1 > a.1 { b } else { a })
                .unwrap();
            start += index + 1;
            max_digit
        })
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
