use std::{collections::BTreeMap, fs::read_to_string};

struct Game<'a> {
    ind: usize,
    draws: Vec<BTreeMap<&'a str, u64>>,
}

pub fn a() {
    let mut sum = 0;
    for line in read_to_string("input_2").unwrap().lines() {
        let game = parse_game(line);
        if !game.draws.iter().any(|m|
                 // Either this handles None corectly
                 // // or there is always an entry for each color
                 m.get("red") > Some(&12)
                 || m.get("green") > Some(&13)
                 || m.get("blue") > Some(&14))
        {
            //println!("{}",game.ind);
            sum += game.ind;
        }
    }
    println!("{}", sum);
}

pub fn b() {
    let mut sum = 0;
    for line in read_to_string("input_2").unwrap().lines() {
        let game = parse_game(line);
        let power = min_for(&game, "green") * min_for(&game, "blue") * min_for(&game, "red");
        //println!("{}",power);
        sum += power;
    }
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
