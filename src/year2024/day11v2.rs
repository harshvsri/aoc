use std::collections::HashMap;

pub fn get_stones() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let mut stone_map = HashMap::new();
    content.split_whitespace().for_each(|num| {
        let val = num
            .parse::<u64>()
            .expect("Should be a valid number to parse.");

        stone_map.entry(val).and_modify(|v| *v += 1).or_insert(1u64);
    });

    let blinks = 75;
    for _ in 1..=blinks {
        blink(&mut stone_map);
    }

    println!(
        "Stone count after {} blinks: {}.",
        blinks,
        stone_map.values().sum::<u64>()
    );
}

fn blink(stone_map: &mut HashMap<u64, u64>) {
    #[inline]
    fn split_stone(num: u64) -> (u64, u64) {
        let x = 10u64.pow((num.ilog10() + 1) / 2);
        (num / x, num % x)
    }

    #[inline]
    fn update(map: &mut HashMap<u64, u64>, key: u64, val: u64) {
        map.entry(key).and_modify(|v| *v += val).or_insert(val);
    }

    let mut next_map = HashMap::with_capacity(stone_map.len());

    for (stone, count) in stone_map.drain() {
        match stone {
            0 => update(&mut next_map, 1, count),
            special_stone if (special_stone.ilog10() + 1) % 2 == 0 => {
                let (s1, s2) = split_stone(special_stone);
                update(&mut next_map, s1, count);
                update(&mut next_map, s2, count);
            }
            _ => update(&mut next_map, stone * 2024, count),
        }
    }

    *stone_map = next_map;
}
