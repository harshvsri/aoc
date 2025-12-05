pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let (fresh_id_ranges, _ids) = data
        .split_once("\n\n")
        .expect("Must contain a valid delemeter[lflf].");

    let mut fresh_id_ranges = fresh_id_ranges
        .lines()
        .map(|l| {
            let (start, end) = l
                .split_once("-")
                .expect("Must contain a valid delemeter[-]");
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();
    fresh_id_ranges.sort();

    let mut prev_end = 0;
    let mut res = 0;
    for (mut start, end) in fresh_id_ranges {
        if start <= prev_end {
            start = prev_end + 1
        }
        if start <= end {
            res += (end - start + 1) as u128;
            prev_end = end;
        }
    }
    println!("Res: {res}");

    // let fresh_ids = ids
    //     .lines()
    //     .map(|l| l.parse::<u64>().unwrap())
    //     .filter(|&id| is_valid_id(&fresh_id_ranges, id))
    //     .count();
    // println!("Fresh ids: {fresh_ids}");
}

pub fn is_valid_id(id_ranges: &[(u64, u64)], id: u64) -> bool {
    id_ranges
        .iter()
        .any(|&(start, end)| id >= start && id <= end)
}
