#[derive(Debug)]
pub struct Machine {
    pub state: Vec<char>,
    pub operations: Vec<Vec<u64>>,
    pub joltages: Vec<u64>,
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
        let joltages = parse(&joltage[1..joltage.len() - 1]);

        Self {
            state,
            operations,
            joltages,
        }
    }

    // INFO: PART I: This completely ignores that we can take any operation multiple times, intentionally to exploit the pattern found.
    // OBSERVATION: If we press any button even times the effect is cancelled and odd presses result in a toggle
    // So keeping that in mind i should only think of a states that can make me reach to desired state in least number of button presses.
    //
    // HACK: PART II: We may need to use combinatorics since the order of the key presse doesnt matters.
    // And we may also introduce a very hard limit of min and max from the joltages.
    // Here i will update the new base case which is that it must never exceed the joltages.
    // In such problems using BFS makes so much sense what we need is the smallest num of the presses.
    pub fn min_operations(&self, op_freq: &mut Vec<usize>, index: usize) -> u64 {
        let mut min = u64::MAX;

        if index == self.operations.len() {
            let mut state = vec!['.'; self.state.len()];
            for (idx, _) in op_freq.iter().enumerate().filter(|(_, f)| **f != 0) {
                for &op in &self.operations[idx] {
                    match state[op as usize] {
                        '.' => state[op as usize] = '#',
                        '#' => state[op as usize] = '.',
                        _ => unreachable!(),
                    }
                }
            }

            if state == self.state {
                min = min.min(op_freq.iter().sum::<usize>() as u64);
            }
            return min;
        }

        op_freq[index] = 1;
        min = min.min(self.min_operations(op_freq, index + 1));
        op_freq[index] = 0;

        min = min.min(self.min_operations(op_freq, index + 1));
        return min;
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");
    let machines = data.lines().map(Machine::new).collect::<Vec<_>>();

    let res = machines
        .iter()
        .map(|m| m.min_operations(&mut vec![0; m.operations.len()], 0))
        .sum::<u64>();
    println!("Final Res: {res}");
}
