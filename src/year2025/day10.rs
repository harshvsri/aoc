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

    // INFO: PART II: We may need to use combinatorics since the order of the key presse doesnt matters.
    // And we may also introduce a very hard limit of min and max from the joltages.
    // Here i will update the new base case which is that it must never exceed the joltages.
    // In such problems using BFS makes so much sense what we need is the smallest num of the presses.
    pub fn min_ops(&self) -> u64 {
        let mut seen = std::collections::HashSet::from([vec![0; self.joltages.len()]]);
        let mut queue = std::collections::VecDeque::from([(0, vec![0; self.joltages.len()])]);
        while let Some((clicks, state)) = queue.pop_front() {
            if state == self.joltages {
                return clicks;
            }

            for op in &self.operations {
                let mut new_state = state.clone();
                for &button in op {
                    new_state[button as usize] += 1;
                }

                if !new_state
                    .iter()
                    .zip(&self.joltages)
                    .any(|(curr, max)| curr > max)
                    && seen.insert(new_state.clone())
                {
                    queue.push_back((clicks + 1, new_state));
                }
            }
        }
        unreachable!("There is no valid solution");
    }

    pub fn min_clicks(&self, clicks: u64, curr: &mut Vec<u64>) -> u64 {
        let mut min = u64::MAX;
        // Need to optimize this later.
        if curr.iter().zip(&self.joltages).any(|(c, o)| c > o) {
            return min;
        }

        println!("{clicks}, {curr:?}");
        if curr == &self.joltages {
            // We have found the best solution.
            return clicks;
        }

        for op in &self.operations {
            for &idx in op {
                curr[idx as usize] += 1;
            }

            min = min.min(self.min_clicks(clicks + 1, curr));

            for &idx in op {
                curr[idx as usize] -= 1;
            }
        }
        return min;
    }
}

// INFO: For PART II we can think of it like a1*x1 + a2*x2 + a3*x3 + a4*x4 + ... = X
// where, a1 + a2 + a3 + a4 + ... = k; k > 0 and we need to minimize this sum.
pub fn solve() {
    let data =
        std::fs::read_to_string("test.txt").expect("File must be present in the root directory.");
    let machines = data.lines().map(Machine::new).collect::<Vec<_>>();

    let res = machines
        .iter()
        .map(|m| dbg!(m.min_clicks(0, &mut vec![0; m.joltages.len()])))
        .sum::<u64>();
    println!("Final Res: {res}");
}
