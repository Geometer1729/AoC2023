use num::integer::lcm;
use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

pub fn a() {
    let txt = read_to_string("input_8").expect("no file or so");
    let mut lines = txt.lines();
    let instructions_raw = lines.next().expect("empty");
    let instructions: Vec<bool> = instructions_raw.chars().map(|c| c == 'R').collect();
    lines.next();
    let mut nodes: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
    for line in lines {
        let (node_raw, rest) = line.split_once(" = (").expect("bad line");
        let (left_raw, right_paren) = rest.split_once(", ").expect("bad line");
        let node = to_id(node_raw);
        // probably a bad way to do it but it shouldn't really matter
        let right_raw = right_paren.split_once(")").expect("bad line").0;
        nodes.insert(node, (to_id(left_raw), to_id(right_raw)));
    }
    let mut pos = to_id("AAA");
    let end = to_id("ZZZ");
    let mut count = 0;
    loop {
        count += 1;
        follow(&mut pos, &nodes, &instructions);
        if pos == end {
            break;
        }
    }
    println!("{}", count * instructions.len());
}

pub fn b() {
    let txt = read_to_string("input_8").expect("no file or so");
    let mut lines = txt.lines();
    let instructions_raw = lines.next().expect("empty");
    let instructions: Vec<bool> = instructions_raw.chars().map(|c| c == 'R').collect();
    lines.next();
    let mut nodes: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
    let mut starts: Vec<u32> = Vec::new();
    let mut ends: BTreeSet<u32> = BTreeSet::new();
    for line in lines {
        let (node_raw, rest) = line.split_once(" = (").expect("bad line");
        let (left_raw, right_paren) = rest.split_once(", ").expect("bad line");
        let node = to_id(node_raw);
        // probably a bad way to do it but it shouldn't really matter
        let right_raw = right_paren.split_once(")").expect("bad line").0;
        nodes.insert(node, (to_id(left_raw), to_id(right_raw)));
        let bytes = node_raw.as_bytes();
        if bytes[2] == ('Z' as u8) {
            ends.insert(node);
        }
        if bytes[2] == ('A' as u8) {
            starts.push(node);
        }
    }
    let mut pos = starts.clone();
    let mut times = Vec::new();
    for mut p in pos.iter_mut() {
        let mut steps = 0;
        loop {
            steps += 1;
            follow(&mut p, &nodes, &instructions);
            if ends.contains(p) {
                times.push(steps);
                break;
            }
        }
    }
    let res: u64 = times.iter().fold(1, |a, &b| lcm(a, b));
    println!("{}", res * instructions.len() as u64);
}

fn follow(pos: &mut u32, nodes: &BTreeMap<u32, (u32, u32)>, steps: &Vec<bool>) {
    for s in steps {
        let &(l, r) = nodes.get(&pos).expect("bad pos");
        if *s {
            *pos = r;
        } else {
            *pos = l;
        }
    }
}

fn to_id(w: &str) -> u32 {
    match w.as_bytes() {
        &[a, b, c] => (a as u32) << 16 | (b as u32) << 8 | (c as u32),
        _ => {
            panic!("bad length")
        }
    }
}
