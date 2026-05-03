use std::collections::HashMap;

struct Operation<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
    res: &'a str,
}

impl<'a> Operation<'a> {
    fn eval(&self, map: &mut HashMap<&'a str, u8>) -> bool {
        if let (Some(a), Some(b)) = (map.get(self.a), map.get(self.b)) {
            map.insert(
                self.res,
                match self.op {
                    "AND" => a & b,
                    "OR" => a | b,
                    "XOR" => a ^ b,
                    _ => panic!("Invalid operator found"),
                },
            );
            true
        } else {
            false
        }
    }
}

pub fn solve() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let (operators, operations) = content
        .split_once("\n\n")
        .expect("Must constain a valid delimeter");

    let mut map = operators
        .lines()
        .map(|line| {
            let (k, v) = line
                .split_once(": ")
                .expect("Must constain a valid delimeter");
            (k, v.parse::<u8>().expect("Must contain exactly 3 symbols."))
        })
        .collect::<HashMap<_, _>>();

    let mut operations = operations
        .lines()
        .map(|line| {
            let (values, res) = line
                .split_once(" -> ")
                .expect("Must constain a valid delimeter");

            let mut values = values.split_whitespace();
            let (a, op, b) = (
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            );
            Operation { a, b, op, res }
        })
        .collect::<Vec<_>>();

    loop {
        operations.retain(|op| !op.eval(&mut map));
        if operations.is_empty() {
            break;
        }
    }

    // let mut res = map
    //     .iter()
    //     .filter(|op| op.0.starts_with("z"))
    //     .collect::<Vec<_>>();
    //
    // res.sort();
    // res.reverse();
    //
    // let decimal = res
    //     .iter()
    //     .fold(0u64, |acc, (_, val)| (acc << 1) | **val as u64);

    let mut decimal = 0u64;
    for i in 0.. {
        // Well this make an allocation per key.
        let key = format!("z{:02}", i);
        if let Some(&val) = map.get(key.as_str()) {
            decimal |= (val as u64) << i;
        } else {
            break; // Stop when we hit a 'z' bit that doesn't exist
        }
    }
    println!("{}", decimal);
}
