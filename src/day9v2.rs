use std::fs;

#[derive(Clone, PartialEq)]
enum Block {
    Free,
    File(u32),
}

pub fn get_checksum() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let data = content.lines().collect::<String>();

    let mut checksum: Vec<Block> = Vec::new();
    let mut id = 0;

    for (index, ch) in data.chars().enumerate() {
        let value_freq = ch
            .to_digit(10)
            .expect("Character should be a valid number.");

        let block = if index % 2 != 0 {
            Block::Free
        } else {
            let file = Block::File(id);
            id += 1;
            file
        };

        add_data(&mut checksum, &block, value_freq);
    }

    adjust_checksum(&mut checksum);
    let checksum = final_checksum(&checksum);
    println!("Final res: {}", checksum);
}

fn add_data(checksum: &mut Vec<Block>, block: &Block, times: u32) {
    for _ in 0..times {
        checksum.push(block.clone());
    }
}

fn adjust_checksum(checksum: &mut Vec<Block>) {
    let (mut left, mut right) = (0, checksum.len() - 1);

    while left < right {
        while left < right && checksum[left] != Block::Free {
            left += 1;
        }
        while left < right && checksum[right] == Block::Free {
            right -= 1;
        }

        if left < right {
            checksum[left] = checksum[right].clone();
            checksum[right] = Block::Free;
            left += 1;
            right -= 1;
        }
    }
}

fn final_checksum(checksum: &Vec<Block>) -> u128 {
    let mut res = 0;

    for (index, block) in checksum.iter().enumerate() {
        let val = match block {
            Block::File(data) => data,
            Block::Free => {
                println!("We are at index({}) and found a Block::Free.", index);
                break;
            }
        };

        res += (index as u128) * (*val as u128);
    }
    return res;
}

fn _display_checksum(checksum: &Vec<Block>) {
    for block in checksum {
        let val = match block {
            Block::File(val) => &val.to_string(),
            Block::Free => ".",
        };
        print!("{}", val);
    }
    println!()
}
