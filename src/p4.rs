use std::fmt::Debug;
use std::{fs::read_to_string, str::FromStr};

pub fn a() {
    let total: u64 = read_to_string("input_4")
        .expect("no file or so")
        .lines()
        .map(|line| {
            let (card_have_raw, wining_raw) = line.split_once("|").expect("no |");
            let have_raw = card_have_raw.split_once(":").expect("no :").1;
            let wining: Vec<u64> = parse_words(wining_raw).collect();
            let have = parse_words(have_raw);
            have.filter(|n| wining.contains(n))
                .count()
                .checked_sub(1)
                .map(|n| 1 << n)
                .unwrap_or(0)
        })
        .sum();
    println!("{}", total)
}

pub fn b() {
    let txt = read_to_string("input_4").expect("no file or so");
    let lines = txt.lines().collect::<Vec<_>>();
    let mut mults = vec![1; lines.len()];
    let total : u64 = lines
        .iter()
        .enumerate()
        .map(|(ind,line)| {
            let (card_have_raw, wining_raw) = line.split_once("|").expect("no |");
            let have_raw = card_have_raw.split_once(":").expect("no :").1;
            let wining: Vec<u64> = parse_words(wining_raw).collect();
            let have = parse_words(have_raw);
            let score = have.filter(|n| wining.contains(n)).count();
            let mult = mults[ind];
            for i in ind + 1..ind + 1 + (score as usize) {
                mults[i] += mult;
            }
            mult
        })
        .sum();
    println!("{}", total)
}

fn parse_words<'a, A>(w: &'a str) -> Box<dyn Iterator<Item = A> + 'a>
where
    A: FromStr,
    <A as FromStr>::Err: Debug,
{
    Box::new(
        w.trim_start()
            .split(" ")
            .filter(|w| w != &"")
            .map(|r| r.parse::<A>().expect("bad int")),
    )
}
