pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let worksheet = data
        .lines()
        .take(data.lines().count() - 1)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let operators = data
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();

    #[inline]
    fn calculate(nums: &[u64], op: &str) -> u64 {
        println!("[{op}] {:?}", nums);
        match op {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product::<u64>(),
            _ => unreachable!(),
        }
    }

    let mut res = 0;
    let mut op_idx = 0;
    let mut nums = vec![];
    for col in 0..worksheet[0].len() {
        let num_str = (0..worksheet.len())
            .map(|row| worksheet[row][col])
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        if num_str.is_empty() {
            res += calculate(&nums, operators[op_idx]);
            nums.clear();
            op_idx += 1;
        } else {
            nums.push(num_str.parse::<u64>().unwrap());
        }
    }

    res += calculate(&nums, operators[op_idx]);
    println!("{res}");
}
