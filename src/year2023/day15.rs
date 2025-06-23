use std::{collections::HashMap, fmt::Debug};

struct Info {
    label: String,
    val: usize,
}

impl Debug for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.label, self.val)
    }
}

fn get_hash(data: &str) -> usize {
    let mut hash = 0;
    for ch in data.chars() {
        hash += (ch as u8) as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn compute_strings() {
    let mut map: HashMap<usize, Vec<Info>> = HashMap::new();

    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .trim()
        .split(",")
        .for_each(|data| {
            if let Some((info_label, info_val)) = data.split_once("=") {
                let info_val = info_val.parse::<usize>().expect("Must be a valid number.");
                let key = get_hash(info_label);
                let val = map.entry(key).or_insert_with(Vec::new);

                if let Some(info) = val.iter_mut().find(|info| info.label == info_label) {
                    info.val = info_val;
                } else {
                    val.push(Info {
                        label: info_label.to_owned(),
                        val: info_val,
                    });
                }
            }

            if let Some((info_label, _)) = data.split_once("-") {
                let key = get_hash(info_label);

                if let Some(val) = map.get_mut(&key) {
                    val.retain(|info| info.label != info_label)
                }
            }
        });

    for (key, val) in map.iter() {
        if !val.is_empty() {
            println!("Box {: >3}: {:?}", key, val);
        }
    }

    let focal_len = map
        .iter()
        .map(|(key, val)| {
            (key + 1)
                * val
                    .iter()
                    .enumerate()
                    .map(|(index, info)| (index + 1) * info.val)
                    .sum::<usize>()
        })
        .sum::<usize>();

    println!("Focal Len -> {focal_len}");
}
