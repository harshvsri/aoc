use std::collections::HashMap;

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

        let mut operations = data[1..data.len() - 1]
            .iter()
            .map(|s| parse(&s[1..s.len() - 1]))
            .collect::<Vec<_>>();
        // Sorting in descending order by the impact aka count of affected indexes.
        // This will help in applying greedy aproach later.
        operations.sort_unstable_by_key(|a| std::cmp::Reverse(a.len()));

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

    // INFO: For PART II we can think of it like a1*x1 + a2*x2 + a3*x3 + a4*x4 + ... = X
    // where, a1 + a2 + a3 + a4 + ... = k; k > 0 and we need to minimize this sum.
    // But as of now this uses GREEDY approach
    pub fn min_clicks(&self, clicks: u64, joltages: &mut Vec<u64>) -> u64 {
        // OPTIMIZATIONS
        //
        // Well we can also make an optimization where we can update the self.joltages
        // and hence no need to maintain a seperate joltages.
        //
        // Also we dont need to keep track of the best clicks since we are returning as soon
        // as we find a valid a valid solution coz of the greedy approach.
        if joltages.iter().zip(&self.joltages).any(|(c, o)| c > o) {
            return u64::MAX;
        }

        if joltages == &self.joltages {
            return clicks;
        }

        for op in &self.operations {
            for &idx in op {
                joltages[idx as usize] += 1;
            }

            // We must prune this heavily and want an early termination
            let res = self.min_clicks(clicks + 1, joltages);
            if res != u64::MAX {
                return res;
            }

            for &idx in op {
                joltages[idx as usize] -= 1;
            }
        }
        return u64::MAX;
    }

    pub fn min_ops_helper(
        &self,
        index: usize,
        ops: &mut Vec<bool>,
        joltage_state: &Vec<char>,
        configs: &mut Vec<Vec<bool>>,
    ) -> u64 {
        let mut min = u64::MAX;

        if index == self.operations.len() {
            let mut state = vec!['.'; self.state.len()];
            for (idx, _) in ops.iter().enumerate().filter(|(_, f)| **f) {
                for &op in &self.operations[idx] {
                    match state[op as usize] {
                        '.' => state[op as usize] = '#',
                        '#' => state[op as usize] = '.',
                        _ => unreachable!(),
                    }
                }
            }

            if &state == joltage_state {
                configs.push(ops.clone());
                min = min.min(ops.iter().filter(|v| **v).count() as u64);
            }
            return min;
        }

        ops[index] = true;
        min = min.min(self.min_ops_helper(index + 1, ops, joltage_state, configs));
        ops[index] = false;

        min = min.min(self.min_ops_helper(index + 1, ops, joltage_state, configs));
        return min;
    }

    pub fn min_joltages_helper(
        &self,
        index: usize,
        ops: &mut Vec<bool>,
        joltage_state: &Vec<u64>,
    ) -> u64 {
        let mut min = u64::MAX;

        if index == self.operations.len() {
            let mut state = vec![0; self.joltages.len()];
            for (idx, _) in ops.iter().enumerate().filter(|(_, f)| **f) {
                for &op in &self.operations[idx] {
                    state[op as usize] += 1;
                }
            }

            if &state == joltage_state {
                min = min.min(ops.iter().filter(|v| **v).count() as u64);
            }
            return min;
        }

        ops[index] = true;
        min = min.min(self.min_joltages_helper(index + 1, ops, joltage_state));
        ops[index] = false;

        min = min.min(self.min_joltages_helper(index + 1, ops, joltage_state));
        return min;
    }

    // NOTE: PROPOSED SOLUTION
    // This methods first try to find all the click patterns to achieve joltages_state
    // {. . . . . } -> ['./#'...]
    // Since every joltages combination can be expressed as a unique state aka joltages_state
    // Which will be achieved in say N diff ways and that clicks are recorded as diff configuration aka configs
    // Now we can try to reduce the remaning joltages { . . . . } into a joltages pattern that can
    // be formed via unique clicks and the by which factor we will reduce the space will be
    // considered while making up the final solution.
    // Addition to this after initial clicks the total clicks and even indivisual clicks are even
    // hence they essentially negate their effect but that doesnt gaurentee that every even click
    // pattern must be a even factor i mean 2,4,4,6,8 can be valid and curr apprach assumes that the
    // clicks must be made by either take or dont so basically [0/1..]*factor which comes out to be
    // [factor, ...] which is not cvertain.
    pub fn solve_smart(&self) -> u64 {
        let num_buttons = self.operations.len();
        let num_variables = self.joltages.len();

        let mut coeffs = vec![vec![0; num_variables]; num_buttons];
        for (i, op) in self.operations.iter().enumerate() {
            for &idx in op {
                coeffs[i][idx as usize] = 1;
            }
        }

        let mut pattern_costs: HashMap<Vec<u64>, HashMap<Vec<u64>, u64>> = HashMap::new();

        let mut all_subsets: Vec<Vec<usize>> = Vec::new();

        fn get_combs(
            start: usize,
            num_buttons: usize,
            current: &mut Vec<usize>,
            all_subsets: &mut Vec<Vec<usize>>,
        ) {
            all_subsets.push(current.clone());
            for i in start..num_buttons {
                current.push(i);
                get_combs(i + 1, num_buttons, current, all_subsets);
                current.pop();
            }
        }

        get_combs(0, num_buttons, &mut Vec::new(), &mut all_subsets);
        all_subsets.sort_unstable_by_key(|s| s.len());

        for buttons in all_subsets {
            let cost = buttons.len() as u64;
            let mut pattern = vec![0; num_variables];
            for &b in &buttons {
                for v in 0..num_variables {
                    pattern[v] += coeffs[b][v];
                }
            }
            let parity_pattern: Vec<u64> = pattern.iter().map(|&x| x % 2).collect();
            let entry = pattern_costs.entry(parity_pattern).or_default();
            if !entry.contains_key(&pattern) {
                entry.insert(pattern, cost);
            }
        }

        let mut memo = HashMap::new();
        self.solve_single_aux(&self.joltages, &pattern_costs, &mut memo)
    }

    fn solve_single_aux(
        &self,
        goal: &[u64],
        pattern_costs: &HashMap<Vec<u64>, HashMap<Vec<u64>, u64>>,
        memo: &mut HashMap<Vec<u64>, u64>,
    ) -> u64 {
        if goal.iter().all(|&x| x == 0) {
            return 0;
        }
        if let Some(&ans) = memo.get(goal) {
            return ans;
        }

        let mut answer = 1_000_000_000;
        let parity: Vec<u64> = goal.iter().map(|&x| x % 2).collect();

        if let Some(patterns) = pattern_costs.get(&parity) {
            for (pattern, &cost) in patterns {
                let mut valid = true;
                for (p, g) in pattern.iter().zip(goal.iter()) {
                    if p > g {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    let new_goal: Vec<u64> = pattern
                        .iter()
                        .zip(goal.iter())
                        .map(|(p, g)| (g - p) / 2)
                        .collect();
                    let sub_ans = self.solve_single_aux(&new_goal, pattern_costs, memo);
                    if sub_ans != 1_000_000_000 {
                        answer = answer.min(cost + 2 * sub_ans);
                    }
                }
            }
        }

        memo.insert(goal.to_vec(), answer);
        answer
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");
    let machines = data.lines().map(Machine::new).collect::<Vec<_>>();

    let res = machines.iter().map(|m| m.solve_smart()).sum::<u64>();
    println!("Final Res: {res}");
}
