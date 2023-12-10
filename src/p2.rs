use std::{collections::BTreeMap, fs::read_to_string};

struct Game<'a> {
    ind: usize,
    draws: Vec<BTreeMap<&'a str, u64>>,
}

pub fn a() {
    let sum: usize = read_to_string("input_2")
        .expect("no file")
        .lines()
        .map(parse_game)
        .filter(|game| {
            game.draws.iter().all(|m| {
                [("red", 12), ("green", 13), ("blue", 14)]
                    .iter()
                    .all(|(color, max)| m.get(color) <= Some(max))
            })
        })
        .map(|game| game.ind)
        .sum();
    println!("{}", sum);
}

pub fn b() {
    let sum: u64 = read_to_string("input_2")
        .expect("no file")
        .lines()
        .map(parse_game)
        .map(|game| {
            ["red", "green", "blue"]
                .iter()
                .map(|color| min_for(&game, color))
                .product::<u64>()
        })
        .sum();
    println!("{}", sum);
}

fn min_for<'a>(game: &Game, str: &str) -> u64 {
    *game
        .draws
        .iter()
        .map(|m| m.get(str).unwrap_or(&0))
        .max()
        .expect("empty game?")
}

fn parse_game(line: &str) -> Game {
    let (game_ind, draws_raw) = line.split_once(':').expect("no :");
    let ind = game_ind
        .split_once(' ')
        .expect("no ' '")
        .1
        .parse()
        .expect("Bad int game ind");
    let draws = draws_raw
        .split(';')
        .map(|w: &str| {
            w.split(',')
                .map(|c: &str| {
                    let (amt, color) = c.trim_start().split_once(' ').expect("no ' '");
                    (
                        color,
                        amt.parse::<u64>()
                            .expect(format!("Bad int {}", amt).as_str()),
                    )
                })
                .collect::<BTreeMap<&str, u64>>()
        })
        .collect::<Vec<_>>();
    Game { ind, draws }
}
