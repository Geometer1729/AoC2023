use std::{collections::BTreeMap, fs::read_to_string};

pub fn a() {
    let txt = read_to_string("input_7").expect("no file or so");
    let mut scores = Vec::new();
    for line in txt.lines() {
        let (hand_raw, bid_raw) = line.split_once(' ').expect("no ' '");
        let hand: Vec<u8> = hand_raw.chars().map(parse_card_a).collect();
        let bid: u64 = bid_raw.parse().expect("bad bid?");
        scores.push((score_a(hand), bid));
    }
    scores.sort();
    let total: usize = scores
        .iter()
        .enumerate()
        .map(|(ind, (_, bid))| (ind + 1) * (*bid as usize))
        .sum();
    println!("{}", total);
}

pub fn b() {
    let txt = read_to_string("input_7").expect("no file or so");
    let mut scores = Vec::new();
    for line in txt.lines() {
        let (hand_raw, bid_raw) = line.split_once(' ').expect("no ' '");
        let hand: Vec<u8> = hand_raw.chars().map(parse_card_b).collect();
        let bid: u64 = bid_raw.parse().expect("bad bid?");
        scores.push((score_b(hand), bid));
    }
    scores.sort();
    let total: usize = scores
        .iter()
        .enumerate()
        .map(|(ind, (_, bid))| (ind + 1) * (*bid as usize))
        .sum();
    println!("{}", total);
}

fn parse_card_a(c: char) -> u8 {
    if '1' <= c && c <= '9' {
        return (c as u8) - '0' as u8;
    }
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("bad card '{}'", c),
    }
}

fn parse_card_b(c: char) -> u8 {
    if '1' <= c && c <= '9' {
        return (c as u8) - '0' as u8;
    }
    match c {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("bad card '{}'", c),
    }
}

fn score_a(hand: Vec<u8>) -> u64 {
    let mut score = 0;
    for i in 0..5 {
        score |= (hand[i] as u64) << 8 * (4 - i)
    }
    let mut count_map = BTreeMap::new();
    for card in hand.clone() {
        match count_map.get_mut(&card) {
            Some(count) => {
                *count += 1;
            }
            None => {
                count_map.insert(card, 1);
            }
        }
    }
    let mut counts = count_map.values().map(|&i| i).collect::<Vec<u64>>();
    counts.sort();
    counts.reverse();
    score |= match counts[..] {
        [5] => 6 << 6 * 8,
        [4, 1] => 5 << 6 * 8,
        [3, 2] => 4 << 6 * 8,
        [3, 1, 1] => 3 << 6 * 8,
        [2, 2, 1] => 2 << 6 * 8,
        [2, 1, 1, 1] => 1 << 6 * 8,
        [1, 1, 1, 1, 1] => 0,
        _ => {
            panic!("bad counts {:#?}", counts)
        }
    };
    score
}

fn score_b(hand: Vec<u8>) -> u64 {
    let mut score = 0;
    for i in 0..5 {
        score |= (hand[i] as u64) << 8 * (4 - i)
    }
    let mut count_map = BTreeMap::new();
    for card in hand.clone() {
        match count_map.get_mut(&card) {
            Some(count) => {
                *count += 1;
            }
            None => {
                count_map.insert(card, 1);
            }
        }
    }
    let js: u64 = *(count_map.get(&1).unwrap_or(&0));
    count_map.remove(&1);
    let mut counts = count_map.values().map(|&i| i).collect::<Vec<u64>>();
    counts.sort();
    counts.reverse();
    match counts.get_mut(0) {
        Some(c) => {
            *c += js;
        }
        None => {
            counts.push(js);
        }
    }
    score |= match counts[..] {
        [5] => 6 << 6 * 8,
        [4, 1] => 5 << 6 * 8,
        [3, 2] => 4 << 6 * 8,
        [3, 1, 1] => 3 << 6 * 8,
        [2, 2, 1] => 2 << 6 * 8,
        [2, 1, 1, 1] => 1 << 6 * 8,
        [1, 1, 1, 1, 1] => 0,
        _ => {
            panic!("bad counts {:#?}", counts)
        }
    };
    score
}
