use std::fs;

pub fn get_stones() {
    let content = fs::read_to_string("input.txt")
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

    let blinks = 40;
    for i in 0..blinks {
        blink(&mut stones);
        if i % 5 == 0 {
            println!("Stone count after {} blinks: {}.", i, stones.len())
        }
    }

    println!("Stone count after {} blinks: {}.", blinks, stones.len());
}

fn blink(stones: &mut Vec<u64>) {
    let mut new_stones: Vec<u64> = Vec::new();

    for stone in stones.iter_mut() {
        if *stone == 0 {
            *stone = 1;
        } else {
            let digits = digit_count(stone.clone());

            if digits % 2 == 0 {
                let (stone1, stone2) = split_stone(stone.clone());
                *stone = stone1;
                new_stones.push(stone2);
            } else {
                *stone *= 2024;
            }
        }
    }
    stones.extend(new_stones);
}

fn digit_count(mut num: u64) -> usize {
    let mut count = 0;
    if num == 0 {
        return 1;
    }
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn split_stone(mut num: u64) -> (u64, u64) {
    let mut digits: Vec<u64> = Vec::new();

    while num > 0 {
        let digit = num % 10;
        digits.push(digit);
        num /= 10;
    }

    assert!(
        digits.len() % 2 == 0,
        "Got odd number of digits but expected even number of digits."
    );

    let num1 = digits[..digits.len() / 2]
        .iter()
        .fold(0, |acc, val| acc * 10 + *val);
    let num2 = digits[digits.len() / 2..]
        .iter()
        .fold(0, |acc, val| acc * 10 + *val);

    (num1, num2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(9), 1);
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(999), 3);
        assert_eq!(digit_count(1000), 4);
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_stone(1234), (12, 34));
        assert_eq!(split_stone(567890), (567, 890));
    }
}
