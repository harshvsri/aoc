#[inline]
fn parse(s: &str) -> u64 {
    s.parse().expect("Must be a valid number.")
}

#[inline]
fn is_repeating(id: &str) -> bool {
    let len = id.len();
    (1..=(len / 2)).filter(|g| len % g == 0).any(|group_size| {
        let pattern = &id[0..group_size];
        (0..len)
            .step_by(group_size)
            .all(|x| &id[x..(x + group_size)] == pattern)
    })
}

pub fn foo() {
    let data =
        std::fs::read_to_string("input.txt").expect("test.txt should be present in the root.");

    let id_ranges = data
        .trim()
        .split(",")
        .map(|range| {
            let (start, end) = range
                .split_once("-")
                .expect("Must contain a valid delemeter.");
            (parse(start), parse(end))
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for (start, end) in id_ranges {
        for num in start..=end {
            if is_repeating(&num.to_string()) {
                println!("Invalid num: {num}");
                res += num;
            }
        }
    }

    println!("Res: {res}");
}
