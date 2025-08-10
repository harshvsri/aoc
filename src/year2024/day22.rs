use std::{
    collections::{HashMap, HashSet},
    usize,
};

const PRUNE_MODULUS: usize = 16777216;
const SECRET_GENERATIONS: usize = 2000;
type SecretNumber = usize;

#[inline(always)]
fn mix(secret: SecretNumber, value: SecretNumber) -> SecretNumber {
    secret ^ value
}

#[inline(always)]
fn prune(secret: SecretNumber) -> SecretNumber {
    secret % PRUNE_MODULUS
}

fn evolve_secret(mut secret: SecretNumber) -> SecretNumber {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
    return secret;
}

pub fn foo() {
    let buyer_prices = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|secret| {
            let mut secret = secret.parse::<usize>().expect("Must be a valid number.");
            let mut prices = vec![secret % 10];

            for _ in 0..SECRET_GENERATIONS {
                secret = evolve_secret(secret);
                let price = secret % 10;
                prices.push(price);
            }
            prices
        })
        .collect::<Vec<_>>();

    let maps = buyer_prices
        .iter()
        .map(|prices| {
            let mut map = HashMap::new();

            for window in prices.windows(5) {
                let seq = [
                    window[1] - window[0],
                    window[2] - window[1],
                    window[3] - window[2],
                    window[4] - window[3],
                ];

                if !map.contains_key(&seq) {
                    map.insert(seq, window.last().unwrap());
                }
            }
            map
        })
        .collect::<Vec<_>>();

    let all_unique_sequences = maps
        .iter()
        .flat_map(|map| map.keys())
        .collect::<HashSet<_>>();

    let max_items = all_unique_sequences
        .iter()
        .map(|&seq| {
            maps.iter()
                .map(|map| {
                    if let Some(&val) = map.get(seq) {
                        val
                    } else {
                        &0usize
                    }
                })
                .sum::<usize>()
        })
        .max()
        .unwrap_or(0);

    println!("{max_items}");
}
