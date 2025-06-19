use std::{cmp::max, fs};

pub fn possible_games() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let games = content
        .lines()
        .map(|line| {
            let (_, cubes) = line
                .split_once(": ")
                .expect("Must contain ': ' as a seperator");
            cubes.split("; ").collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res = games
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|round| GameRound::new(round).is_valid_round())
        })
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    println!("Result: {}", res);

    let sum_of_power = games
        .iter()
        .map(|game| {
            let rounds = game
                .iter()
                .map(|round| GameRound::new(round))
                .collect::<Vec<_>>();

            let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);
            for round in rounds {
                min_red = max(min_red, round.red);
                min_blue = max(min_blue, round.blue);
                min_green = max(min_green, round.green);
            }

            min_red * min_blue * min_green
        })
        .sum::<usize>();

    println!("Sum of power: {}", sum_of_power);
}

#[derive(Debug)]
struct GameRound {
    blue: usize,
    red: usize,
    green: usize,
}

impl GameRound {
    fn new(round: &str) -> Self {
        let mut game_round = GameRound {
            green: 0,
            red: 0,
            blue: 0,
        };

        let value_color_pairs = round
            .split(", ")
            .map(|color| {
                let (value, color_name) = color.split_once(" ").unwrap();
                (value, color_name)
            })
            .collect::<Vec<_>>();

        for (value, color) in value_color_pairs {
            match color {
                "blue" => game_round.blue = value.parse::<usize>().expect("Must be a valid num."),
                "red" => game_round.red = value.parse::<usize>().expect("Must be a valid num."),
                "green" => game_round.green = value.parse::<usize>().expect("Must be a valid num."),
                _ => panic!("Invalid color provided."),
            }
        }

        game_round
    }

    fn is_valid_round(self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
