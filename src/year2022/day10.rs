struct CPU {
    register: i32,
    cycle_count: i32,
    res: i32,
}

impl CPU {
    fn new() -> Self {
        Self {
            register: 1,
            cycle_count: 0,
            res: 0,
        }
    }

    fn valid_cycle(cycle: i32) -> bool {
        if cycle < 20 {
            return false;
        }
        (cycle - 20) % 40 == 0
    }

    fn tick(&mut self, crt: &mut Vec<Vec<char>>) {
        let row = (self.cycle_count / 40) as usize;
        let col = (self.cycle_count % 40) as usize;

        // If the CRT's drawing column (col) falls within the sprite's bounds (X-1, X, X+1),
        if (col as i32 - self.register).abs() <= 1 {
            if row < crt.len() && col < crt[0].len() {
                crt[row][col] = '#';
            }
        }

        self.cycle_count += 1;
        if Self::valid_cycle(self.cycle_count) {
            self.res += self.cycle_count * self.register;
        }
    }

    fn operate(&mut self, ops: Vec<Operation>, crt: &mut Vec<Vec<char>>) {
        for op in ops {
            match op {
                Operation::Noop => {
                    self.tick(crt);
                }
                Operation::Addx(val) => {
                    self.tick(crt);
                    self.tick(crt);
                    self.register += val;
                }
            }
        }
    }
}

enum Operation {
    Noop,
    Addx(i32),
}

impl Operation {
    fn parse(line: &str) -> Self {
        match &line[..4] {
            "addx" => {
                let val = &line[4 + 1..]
                    .parse::<i32>()
                    .expect("Must be a valid number.");
                Operation::Addx(*val)
            }
            "noop" => Operation::Noop,
            _ => panic!("Invalid instruction found."),
        }
    }
}

pub const SPRITE: [char; 3] = ['#'; 3];
pub fn solve() {
    let data = std::fs::read_to_string("input.txt").expect("Must be a valid path.");
    let ops = data.lines().map(Operation::parse).collect::<Vec<_>>();

    let mut cpu = CPU::new();
    let mut crt = vec![vec!['.'; 40]; 6];
    cpu.operate(ops, &mut crt);
    println!("Result: {}", cpu.res);

    for row in crt {
        println!("{}", row.iter().collect::<String>());
    }
}
