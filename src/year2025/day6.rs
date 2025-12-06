pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");
    let worksheet = data.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let operators = worksheet[worksheet.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let operands = worksheet[..worksheet.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|op| op.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for col in 0..operands[0].len() {
        let mut a = operands[0][col];
        for row in 1..operands.len() {
            let b = operands[row][col];
            a = operate(a, b, operators[col]);
        }
        res += a;
    }
    println!("{res}");
}

fn operate(a: u64, b: u64, op: &str) -> u64 {
    match op {
        "+" => a + b,
        "*" => a * b,
        _ => unreachable!(),
    }
}
