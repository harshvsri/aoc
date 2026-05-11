#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn parse(s: &str) -> u64 {
        s.split_whitespace().last().unwrap().parse::<u64>().unwrap()
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(x) => old + x,
            Operation::Mul(x) => old * x,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    true_index: usize,
    false_index: usize,
}

impl Test {
    fn parse(s: &str) -> usize {
        s.split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }

    fn apply(&self, old: u64) -> usize {
        if old % self.divisor == 0 {
            self.true_index
        } else {
            self.false_index
        }
    }
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

const ROUNDS: usize = 10000;

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let mut monkeys = data
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();
            let _monkey_index = lines.next();
            let starting_items = lines
                .next()
                .unwrap()
                .split_once(":")
                .unwrap()
                .1
                .split(",")
                .map(|val| val.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let operation = match lines.next().unwrap().split_once("old ").unwrap().1 {
                "* old" => Operation::Square,
                op => {
                    if op.starts_with("+") {
                        Operation::Add(Operation::parse(op))
                    } else if op.starts_with("*") {
                        Operation::Mul(Operation::parse(op))
                    } else {
                        unreachable!()
                    }
                }
            };

            let test = Test {
                divisor: Test::parse(lines.next().unwrap()) as u64,
                true_index: Test::parse(lines.next().unwrap()),
                false_index: Test::parse(lines.next().unwrap()),
            };

            Monkey {
                starting_items,
                operation,
                test,
                inspection_count: 0,
            }
        })
        .collect::<Vec<_>>();

    let common_divisor = monkeys.iter().map(|m| m.test.divisor).product::<u64>();

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].starting_items);
            for item in items {
                monkeys[i].inspection_count += 1;

                // To prevent the worry levels from growing infinitely (and overflowing),
                // we modulo the new worry level by the product of all monkeys' divisors.
                // Because all divisors are prime numbers, their product is exactly their
                // Lowest Common Multiple (LCM).
                // Modulo by a common multiple preserves the truth value of every monkey's
                // individual divisibility test (`worry % monkey_divisor == 0`).
                let new_worry = monkeys[i].operation.apply(item) % common_divisor;

                let new_idx = monkeys[i].test.apply(new_worry);

                monkeys[new_idx].starting_items.push(new_worry);
            }
        }
    }

    println!("Monkeys after {ROUNDS} rounds.");
    for m in &monkeys {
        println!("{:>8} [ ... ]", m.inspection_count);
    }

    monkeys.sort_unstable_by_key(|m| m.inspection_count);
    monkeys.reverse();
    println!(
        "Res: {}",
        monkeys[0].inspection_count * monkeys[1].inspection_count
    );
}
