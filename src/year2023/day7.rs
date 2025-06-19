use std::{cmp::Ordering, collections::HashMap, fmt::Display, fs::read_to_string};

fn get_file_content() -> String {
    read_to_string("input.txt").expect("input.txt must be present in the root of the directory.")
}

#[derive(Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_rank(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

struct Hand {
    cards: Vec<char>,
    bid: usize,
    rank: usize,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Cards: {:?}  Bid: {}]", self.cards, self.bid,)
    }
}

impl Hand {
    fn new(cards: Vec<char>, bid: usize) -> Hand {
        Hand {
            cards: cards.clone(),
            bid,
            rank: Hand::evaluate_hand(&cards).get_rank(),
        }
    }

    fn evaluate_hand(cards: &Vec<char>) -> HandType {
        let mut card_map = HashMap::new();
        let mut joker_count = 0;

        for card_char in cards {
            if *card_char != 'J' {
                *card_map.entry(card_char).or_insert(0u8) += 1u8;
            } else {
                joker_count += 1;
            }
        }

        let mut counts = card_map.values().map(|val| *val).collect::<Vec<_>>();

        if counts.is_empty() {
            counts.push(joker_count);
        } else {
            // sort_unstable for potentially better performance if order for equal elements doesn't matter.
            counts.sort_unstable();

            let last_count = counts
                .last_mut()
                .expect("Must contain at least one element.");

            *last_count += joker_count;
        }

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn get_card_numeric_value(card_char: char) -> u8 {
        match card_char {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,                                            // Ten
            'J' => 1,                                             // Jack (prev 11)
            'Q' => 12,                                            // Queen
            'K' => 13,                                            // King
            'A' => 14,                                            // Ace
            _ => panic!("Invalid card character: {}", card_char), // Handle unexpected characters
        }
    }
}

pub fn bluff() {
    let content = get_file_content();

    let mut hands = content
        .lines()
        .map(|line| {
            let (cards, bid_amount) = line
                .split_once(" ")
                .expect("Must contain a valid seperstor");

            Hand::new(
                cards.chars().collect::<Vec<_>>(),
                bid_amount.parse::<usize>().expect("Must be a valid number"),
            )
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        // 1. Primary Sort: Compare by rank
        let rank_cmp = a.rank.cmp(&b.rank);
        if rank_cmp != Ordering::Equal {
            return rank_cmp;
        }

        // 2. Secondary Sort (Tie-breaker): Ranks are equal, compare by cards
        for i in 0..a.cards.len() {
            let card_a_val = Hand::get_card_numeric_value(a.cards[i]);
            let card_b_val = Hand::get_card_numeric_value(b.cards[i]);

            let card_cmp = card_a_val.cmp(&card_b_val);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        // 3. Tertiary Sort (If ranks and all cards are identical): Compare by bid (or hand_type,etc.)
        a.bid.cmp(&b.bid)
    });

    let mut total_winning = 0;
    for (index, hand) in hands.iter().enumerate() {
        total_winning += (index + 1) * hand.bid;
        //println!("{}. {hand}", index + 1);
    }
    println!("Total Winning: {}", total_winning);
}
