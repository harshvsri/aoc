#[derive(Debug)]
struct Machine {
    state: Vec<char>,
    operations: Vec<Vec<u64>>,
    _joltages: Vec<u64>,
}

impl Machine {
    fn new(s: &str) -> Self {
        fn parse(s: &str) -> Vec<u64> {
            s.split(",")
                .map(|n| n.parse().expect("Must be a valid number."))
                .collect()
        }

        let data = s.split_whitespace().collect::<Vec<_>>();

        let state = data[0];
        let state = state[1..state.len() - 1].chars().collect::<Vec<_>>();

        let operations = data[1..data.len() - 1]
            .iter()
            .map(|s| parse(&s[1..s.len() - 1]))
            .collect::<Vec<_>>();

        let joltage = data[data.len() - 1];
        let _joltages = parse(&joltage[1..joltage.len() - 1]);

        Self {
            state,
            operations,
            _joltages,
        }
    }

    fn min_operations(&self, groups: &mut Vec<usize>, index: usize) -> u64 {
        let mut min = u64::MAX;
        if index == self.operations.len() {
            let mut state = vec!['.'; self.state.len()];
            for group_idx in groups.iter() {
                for &op in &self.operations[*group_idx] {
                    match state[op as usize] {
                        '.' => state[op as usize] = '#',
                        '#' => state[op as usize] = '.',
                        _ => unreachable!(),
                    }
                }
            }
            if state == self.state {
                min = min.min(groups.len() as u64);
            }
            return min;
        }

        groups.push(index);
        min = min.min(self.min_operations(groups, index + 1));
        groups.pop();
        min = min.min(self.min_operations(groups, index + 1));
        return min;
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");
    let machines = data.lines().map(Machine::new).collect::<Vec<_>>();

    let res = machines
        .iter()
        .map(|m| m.min_operations(&mut Vec::with_capacity(m.operations.len()), 0))
        .sum::<u64>();
    println!("Final Res: {res}");
}
