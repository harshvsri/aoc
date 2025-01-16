use std::fs;

pub fn get_checksum() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let data = content.lines().map(|line| line).collect::<String>();
    println!("Len: {}", data.len());

    let mut checksum = String::new();
    let mut id = -1;

    for (index, ch) in data.chars().enumerate() {
        let value_freq = ch
            .to_digit(10)
            .expect("Character should be a valid number.");

        let value = if index % 2 != 0 {
            "."
        } else {
            id += 1;
            &id.to_string()
        };

        add_data(&mut checksum, value, value_freq);
    }

    println!("Checksum: {}\n\n", checksum);

    let adjusted_checksum = adjust_checksum(&mut checksum);
    println!("Adjusted checksum: {}\n\n", adjusted_checksum);

    let final_checksum = final_checksum(&adjusted_checksum);
    println!("Final checksum: {}", final_checksum);
}

fn add_data(checksum: &mut String, value: &str, times: u32) {
    for _ in 0..times {
        checksum.push_str(value);
    }
}

fn adjust_checksum(checksum: &mut String) -> String {
    let mut chars = checksum.chars().collect::<Vec<char>>();

    let (mut left, mut right) = (0, checksum.len() - 1);

    while left < right {
        while left < right && chars[left] != '.' {
            left += 1;
        }
        while left < right && chars[right] == '.' {
            right -= 1;
        }

        if left < right {
            chars[left] = chars[right];
            chars[right] = '.';
            left += 1;
            right -= 1;
        }
    }

    chars.iter().map(|ch| ch).collect::<String>()
}

fn final_checksum(checksum: &String) -> u128 {
    let mut res = 0;

    for (index, ch) in checksum.chars().enumerate() {
        if ch == '.' {
            println!("We are at index[{}] and found a Dot['.']", index);
            break;
        }

        let ch = ch
            .to_digit(10)
            .expect("Character should be a valid number.");
        res += (index as u128) * (ch as u128);
    }
    return res;
}
