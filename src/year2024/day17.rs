#[derive(Debug)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instuction_ptr: usize,
    program: Vec<usize>,
    result: Vec<usize>,
}

impl Computer {
    fn new(content: &str) -> Self {
        let (reg, program) = content
            .split_once("\n\n")
            .expect("Must contain valid seperator.");
        let mut reg = reg.lines();

        fn parse_reg(reg_str: &str) -> usize {
            reg_str
                .split_once(": ")
                .expect("Register string must contain ': ' as seperator.")
                .1
                .trim()
                .parse::<usize>()
                .expect("Register must contain a valid number.")
        }

        Computer {
            reg_a: parse_reg(reg.next().unwrap()),
            reg_b: parse_reg(reg.next().unwrap()),
            reg_c: parse_reg(reg.next().unwrap()),
            instuction_ptr: 0,
            program: program
                .split_once(": ")
                .expect("Program string must contain ': ' as seperator.")
                .1
                .trim()
                .split(",")
                .map(|op| op.parse::<usize>().expect("op must be a valid number."))
                .collect::<Vec<_>>(),
            result: Vec::new(),
        }
    }

    fn compute(&mut self) {
        loop {
            if self.instuction_ptr >= self.program.len() - 1 {
                break;
            }

            let (opcode, operand) = (
                self.program[self.instuction_ptr],
                self.program[self.instuction_ptr + 1],
            );
            if let Some(val) = self.perform_operation(opcode, operand) {
                self.result.push(val);
            }
            if opcode != 3 || (opcode == 3 && self.reg_a == 0) {
                self.instuction_ptr += 2;
            }
        }
    }

    fn get_combo(&self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand found, [0,6] is valid range."),
        }
    }

    fn perform_operation(&mut self, opcode: usize, operand: usize) -> Option<usize> {
        match opcode {
            0 => {
                self.reg_a /= 2usize.pow(self.get_combo(operand) as u32);
                None
            }
            1 => {
                self.reg_b ^= operand;
                None
            }
            2 => {
                self.reg_b = self.get_combo(operand) % 8;
                None
            }
            3 => {
                if self.reg_a != 0 {
                    self.instuction_ptr = operand;
                }
                None
            }
            4 => {
                self.reg_b ^= self.reg_c;
                None
            }
            5 => Some(self.get_combo(operand) % 8),
            6 => {
                self.reg_b = self.reg_a / 2usize.pow(self.get_combo(operand) as u32);
                None
            }
            7 => {
                self.reg_c = self.reg_a / 2usize.pow(self.get_combo(operand) as u32);
                None
            }
            _ => panic!("Invalid opcode found, [0,7] is valid range."),
        }
    }

    fn reset(&mut self) {
        self.reg_a = 1;
        self.reg_b = 0;
        self.reg_c = 0;
        self.instuction_ptr = 0;
        self.result.clear();
    }

    fn find_initial_value(&mut self, current_a: usize, depth: usize) -> Option<usize> {
        if depth == self.program.len() {
            return Some(current_a);
        }

        for i in 0..8 {
            self.reset();
            // Shift current A left by 3 bits (multiply by 8) and add the current guess `i`.
            let next_a = (current_a << 3) | i;
            self.reg_a = next_a;
            self.compute();

            if self.result == &self.program[(self.program.len() - 1 - depth)..] {
                if let Some(ans) = self.find_initial_value(next_a, depth + 1) {
                    return Some(ans);
                }
            }
        }

        None
    }
}

pub fn solve() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut computer = Computer::new(&content);
    let initial_value = computer
        .find_initial_value(0, 0)
        .expect("No value found for register.");
    println!("{initial_value}");
}
