use std::{fmt::Display, fs};

#[derive(Clone, Debug, PartialEq)]
enum Block {
    Free,
    File(u32),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "{:>3}", "."),
            Self::File(id) => write!(f, "{:>3}", id),
        }
    }
}

pub fn get_checksum() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let data = content.lines().collect::<String>();
    let mut checksum = caclulate_checksum(&data);
    adjust_checksum(&mut checksum);
    println!("Final res: {:?}", checksum_hash(&checksum));
}

fn caclulate_checksum(data: &str) -> Vec<Block> {
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

        for _ in 0..value_freq {
            checksum.push(block.clone());
        }
    }
    checksum
}

fn adjust_checksum(checksum: &mut Vec<Block>) {
    let mut data_chunks = vec![];
    let mut free_chunks = vec![];

    let (mut ptr, mut size, mut prev_block) = (0, 0, &checksum[0]);
    while ptr < checksum.len() {
        if &checksum[ptr] == prev_block {
            size += 1;
        } else {
            match prev_block {
                Block::Free => free_chunks.push((size, ptr - size)),
                Block::File(_) => data_chunks.push((size, ptr - size)),
            }
            prev_block = &checksum[ptr];
            size = 1;
        }
        ptr += 1;
    }
    match prev_block {
        Block::Free => free_chunks.push((size, ptr - size)),
        Block::File(_) => data_chunks.push((size, ptr - size)),
    }

    data_chunks.reverse();
    for (d_size, d_start) in &data_chunks {
        for (f_size, f_start) in &mut free_chunks {
            if d_start < f_start {
                break;
            }
            if d_size <= f_size {
                // Copy the data.
                for i in 0..*d_size {
                    checksum[*f_start + i] = checksum[*d_start + i].clone();
                    checksum[*d_start + i] = Block::Free;
                }
                // Now we need to re arrange the free size.
                *f_size -= d_size;
                *f_start += d_size;
                break;
            }
        }
    }
}

fn checksum_hash(checksum: &Vec<Block>) -> u128 {
    checksum
        .iter()
        .enumerate()
        .map(|(index, block)| {
            let val = match block {
                Block::File(data) => *data,
                Block::Free => 0,
            };
            val as u128 * index as u128
        })
        .sum::<u128>()
}
