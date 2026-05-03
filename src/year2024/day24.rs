use std::collections::HashMap;

#[derive(Clone)]
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

    let operations = operations
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

    // --- PART 1 ---
    let mut operations_clone = operations.clone();
    loop {
        operations_clone.retain(|op| !op.eval(&mut map));
        if operations_clone.is_empty() {
            break;
        }
    }

    let mut decimal = 0u64;
    for i in 0.. {
        let key = format!("z{:02}", i);
        if let Some(&val) = map.get(key.as_str()) {
            decimal |= (val as u64) << i;
        } else {
            break; // Stop when we hit a 'z' bit that doesn't exist
        }
    }
    println!("{}", decimal);

    // --- PART 2 ---
    // A correctly wired Full Adder for bit `i` MUST follow these structural rules:
    // 1. x[i] XOR y[i] -> intermediate_xor
    // 2. x[i] AND y[i] -> intermediate_and
    // 3. intermediate_xor XOR carry_in -> z[i]  (This produces the final sum bit)
    // 4. intermediate_xor AND carry_in -> intermediate_carry
    // 5. intermediate_and OR intermediate_carry -> carry_out (This goes to the next bit)
    //
    // By checking each gate against these rules, we can find the wires that were swapped.

    let mut wrong = Vec::new();

    let highest_z = operations
        .iter()
        .filter(|op| op.res.starts_with('z'))
        .map(|op| op.res)
        .max()
        .unwrap();

    for op in &operations {
        if op.res.starts_with('z') && op.op != "XOR" && op.res != highest_z {
            wrong.push(op.res);
        }

        if op.op == "XOR"
            && !op.res.starts_with('z')
            && !op.a.starts_with('x')
            && !op.a.starts_with('y')
            && !op.b.starts_with('x')
            && !op.b.starts_with('y')
        {
            wrong.push(op.res);
        }

        if op.op == "AND" && op.a != "x00" && op.b != "x00" {
            for sub_op in &operations {
                if (sub_op.a == op.res || sub_op.b == op.res) && sub_op.op != "OR" {
                    wrong.push(op.res);
                }
            }
        }

        if op.op == "XOR" {
            let is_xy_input = (op.a.starts_with('x') || op.b.starts_with('x'))
                && (op.a.starts_with('y') || op.b.starts_with('y'));

            if is_xy_input && !op.a.ends_with("00") && !op.b.ends_with("00") {
                let mut feeds_into_xor = false;
                for sub_op in &operations {
                    if (sub_op.a == op.res || sub_op.b == op.res) && sub_op.op == "XOR" {
                        feeds_into_xor = true;
                        break;
                    }
                }
                if !feeds_into_xor {
                    wrong.push(op.res);
                }
            }
        }
    }

    wrong.sort();
    wrong.dedup();
    println!("{}", wrong.join(","));
}
