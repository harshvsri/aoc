use std::collections::HashMap;

const CONTENT: &'static str = include_str!("../../input.txt");

struct Operation {
    a: &'static str,
    b: &'static str,
    op: &'static str,
    res: &'static str,
}

impl Operation {
    fn eval(&self, map: &mut HashMap<&'static str, u8>) -> bool {
        if let Some(_) = map.get(self.res) {
            return true;
        }
        let (a, b) = match (map.get(self.a), map.get(self.b)) {
            (Some(a), Some(b)) => (a, b),
            _ => return false,
        };

        map.insert(
            self.res,
            match self.op {
                "AND" => a & b,
                "OR" => a | b,
                "XOR" => a ^ b,
                _ => panic!("Invalid operator found"),
            },
        );

        return true;
    }
}

pub fn foo() {
    let (operators, operations) = CONTENT
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
            let (l, res) = line
                .split_once(" -> ")
                .expect("Must constain a valid delimeter");

            let [a, op, b]: [&str; 3] = l
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Must contain exactly 3 symbols.");
            Operation { a, b, op, res }
        })
        .collect::<Vec<_>>();

    loop {
        let pending_ops = operations.len();
        operations.retain(|op| !op.eval(&mut map));
        if operations.is_empty() {
            break;
        }
        if operations.len() == pending_ops {
            println!("DEADLOCK!!!");
            break;
        }
    }

    let mut res = map
        .iter()
        .filter(|op| op.0.starts_with("z"))
        .collect::<Vec<_>>();
    res.sort();
    res.reverse();
    println!("{:?}", res);

    let decimal = res.iter().fold(0u64, |acc, x| (acc << 1) | *x.1 as u64);
    println!("{}", decimal);
}
