pub fn get_stones() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut stones = content
        .split_whitespace()
        .map(|num| {
            let val = num
                .parse::<u64>()
                .expect("Should be a valid number to parse.");
            val
        })
        .collect::<Vec<_>>();

    println!("Stones : {:?}", stones);

    let blinks = 50;
    for i in 1..=blinks {
        blink(&mut stones);
        if i % 5 == 0 {
            println!("Stone count after {} blinks: {}.", blinks, stones.len());
        }
    }

    println!("Stone count after {} blinks: {}.", blinks, stones.len());
}

fn blink(stones: &mut Vec<u64>) {
    #[inline]
    fn split_stone(num: u64) -> (u64, u64) {
        let x = 10u64.pow((num.ilog10() + 1) / 2);
        (num / x, num % x)
    }

    let mut new_stones: Vec<u64> = Vec::new();

    for stone in stones.iter_mut() {
        match stone {
            0 => *stone = 1,
            x if (x.ilog10() + 1) % 2 == 0 => {
                let (s1, s2) = split_stone(*x);
                *x = s1;
                new_stones.push(s2);
            }
            _ => *stone *= 2024,
        }
    }
    stones.extend(new_stones);
}
