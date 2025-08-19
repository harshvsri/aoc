const PIN_SIZE: usize = 5;
pub fn foo() {
    let (mut locks, mut keys) = (Vec::new(), Vec::new());

    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .split("\n\n")
        .for_each(|obj| match obj.chars().take(PIN_SIZE).all(|c| c == '.') {
            true => {
                let mut key = [0; PIN_SIZE];
                obj.lines().take(6).for_each(|line| {
                    for (idx, c) in line.chars().enumerate() {
                        key[idx] += (c == '#') as u8;
                    }
                });
                keys.push(key);
            }
            false => {
                let mut lock = [0; PIN_SIZE];
                obj.lines().skip(1).for_each(|line| {
                    for (idx, c) in line.chars().enumerate() {
                        lock[idx] += (c == '#') as u8;
                    }
                });
                locks.push(lock);
            }
        });

    let mut valid_pairs = 0;
    for lock in &locks {
        for key in &keys {
            valid_pairs += lock.iter().zip(key).all(|(l, k)| l + k <= PIN_SIZE as u8) as usize;
        }
    }
    println!("Found {valid_pairs} valid k-v pair.");
}
