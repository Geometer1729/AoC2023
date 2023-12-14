use std::{cmp::max, fs::read_to_string};

pub fn a() {
    let blocks = parse();
    let total = blocks.iter().map(score).sum::<usize>();
    println!("{total}");
}

pub fn b() {
    let blocks = parse();
    let total = blocks.iter().map(score_smudge).sum::<usize>();
    println!("{total}");
}

fn parse() -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    let mut next = Vec::new();
    for line in read_to_string("input_13").expect("no file").lines() {
        if line == "" {
            ret.push(next);
            next = Vec::new();
        } else {
            next.push(line.to_string());
        }
    }
    ret.push(next);
    ret
}

fn score(block: &Vec<String>) -> usize {
    let h = try_find(block);
    let transposed: Vec<String> = (0..block[0].len())
        .map(|i| {
            block
                .iter()
                .map(|line| line.chars().nth(i).expect("string to short?"))
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let v = try_find(&transposed);
    100 * h + v
}

fn try_find(block: &Vec<String>) -> usize {
    let l = block.len();
    for t in 0..=(l - 2) {
        let tot = 1 + 2 * t;
        if (max(0, tot.checked_sub(l - 1).unwrap_or(0))..=tot / 2)
            .all(|i| block[i] == block[tot - i])
        {
            return (tot + 1) / 2;
        }
    }
    0
}

fn score_smudge(block: &Vec<String>) -> usize {
    let h = try_find_smudge(block);
    let transposed: Vec<String> = (0..block[0].len())
        .map(|i| {
            block
                .iter()
                .map(|line| line.chars().nth(i).expect("string to short?"))
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let v = try_find_smudge(&transposed);
    100 * h + v
}

fn try_find_smudge(block: &Vec<String>) -> usize {
    let l = block.len();
    for t in 0..=(l - 2) {
        let tot = 1 + 2 * t;
        let dist = (max(0, tot.checked_sub(l - 1).unwrap_or(0))..=tot / 2)
            .map(|i| ham(&block[i], &block[tot - i]))
            .sum::<usize>();
        if dist == 1 {
            return (tot + 1) / 2;
        }
    }
    0
}

fn ham(w1: &String, w2: &String) -> usize {
    w1.chars()
        .zip(w2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}
