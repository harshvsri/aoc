use std::{collections::HashSet, fs};

pub fn get_total_points() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let mut scratchcards_matchings = Vec::new();

    let scratchcards_points = content
        .lines()
        .map(|line| {
            let (_, data) = line
                .split_once(":")
                .expect("Must have a valid saperator (:)");

            let data = data.trim();

            let (winning_numbers, random_numbers) = data
                .split_once(" | ")
                .expect("Must have a valid saperator ( | )");

            let (winning_numbers, random_numbers) = (
                parse_numbers(winning_numbers),
                parse_numbers(random_numbers),
            );

            let matching_cards = winning_numbers.intersection(&random_numbers).count();
            scratchcards_matchings.push(matching_cards);

            // Points per card.
            if matching_cards <= 1 {
                matching_cards as u32
            } else {
                2u32.pow((matching_cards - 1) as u32)
            }
        })
        .sum::<u32>();

    println!("Scratchcards Points: {:?}", scratchcards_points);

    let mut scratchcards = vec![1; scratchcards_matchings.len()];
    for index in 0..scratchcards.len() {
        let matching_cards = scratchcards_matchings[index];
        let card = scratchcards[index];

        for idx in index + 1..=index + matching_cards {
            if idx < scratchcards.len() {
                scratchcards[idx] += card;
            }
        }
    }

    println!("Scratchcards Count: {}", scratchcards.iter().sum::<usize>());
}

fn parse_numbers(numbers: &str) -> HashSet<u8> {
    numbers
        .split_whitespace()
        .map(|num| {
            num.parse::<u8>()
                .expect("Must be a valid number in string form.")
        })
        .collect::<HashSet<_>>()
}
