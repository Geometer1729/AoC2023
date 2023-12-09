use std::{
    cmp::{max, min},
    fs::read_to_string,
};

pub fn a() {
    let (seeds, maps) = parse_puzzle();
    let mut outs = Vec::new();
    for seed in seeds {
        let mut pos = seed;
        for map in &maps {
            pos = aply_map(pos, map);
        }
        outs.push(pos);
    }
    println!("{}", outs.iter().min().expect("empty"));
}

pub fn b() {
    let (seeds_raw, maps) = parse_puzzle();
    let mut seeds = Vec::new();
    let mut seed_vals = seeds_raw.iter();
    loop {
        let l = match seed_vals.next() {
            None => {
                break;
            }
            Some(l) => l,
        };
        let r = seed_vals.next().expect("odd seeds");
        seeds.push((*l, *l + *r))
    }
    let mut layer: Vec<Interval> = seeds;
    for map in maps {
        let mut new = Vec::new();
        for i in layer {
            for out in aply_map_interval(i, &map) {
                new.push(out);
            }
        }
        layer = new;
    }
    println!("{}", layer.iter().map(|x| x.0).min().expect("empty"));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    in_start: u64,
    out_start: u64,
    size: u64,
}

type Interval = (u64, u64);

fn aply_map_interval((int_l, int_r): Interval, map: &[Range]) -> Vec<Interval> {
    if map.len() == 0 {
        return vec![(int_l, int_r)];
    }
    let mid = map.len() / 2;
    let range = map[mid];
    let range_l = range.in_start;
    let range_r = range.in_start + range.size - 1;
    let left_side = if int_l < range_l {
        Some((int_l, min(int_r, range_l - 1)))
    } else {
        None
    };
    let right_side = if int_r > range_r {
        Some((max(int_l, range_r + 1), int_r))
    } else {
        None
    };
    let middle_l = max(int_l, range_l);
    let middle_r = min(int_r, range_r);
    let middle = if middle_l <= middle_r {
        Some((
            middle_l + range.out_start - range.in_start,
            middle_r + range.out_start - range.in_start,
        ))
    } else {
        None
    };
    let lefts = left_side.map(|i| aply_map_interval(i, &map[0..mid]));
    let rights = right_side.map(|i| aply_map_interval(i, &map[mid + 1..]));
    let mut ret = match (lefts, rights) {
        (Some(mut l), Some(mut r)) => {
            l.append(&mut r);
            l
        }
        (None, Some(r)) => r,
        (Some(l), None) => l,
        (None, None) => vec![],
    };
    match middle {
        None => {}
        Some(i) => {
            ret.push(i);
        }
    };
    ret
}

fn aply_map(seed: u64, map: &Vec<Range>) -> u64 {
    let mut l = 0;
    let mut r = map.len() - 1;
    loop {
        if r < l {
            return seed;
        }
        let cur = (l + r) / 2;
        let range = map[cur];
        if seed < range.in_start {
            if cur == 0 {
                // it would probably be better to just use signed ints
                return seed;
            }
            r = cur - 1;
        } else if range.in_start + range.size <= seed {
            l = cur + 1;
        } else {
            return seed + range.out_start - range.in_start;
        }
    }
}

fn parse_puzzle() -> (Vec<u64>, Vec<Vec<Range>>) {
    let txt = read_to_string("input_5").expect("no file or so");
    let mut lines = txt.lines();
    let seeds: Vec<u64> = lines
        .next()
        .expect("empty file")
        .split_once(':')
        .expect("no :")
        .1
        .trim_start()
        .split(" ")
        .map(|w| w.parse().expect("bad int"))
        .collect();
    lines.next();
    lines.next();
    let mut maps_raw: Vec<Vec<Vec<u64>>> = Vec::new();
    maps_raw.push(Vec::new());
    let mut skip_next = false;
    for line in lines {
        if line == "" {
            skip_next = true;
            maps_raw.push(Vec::new());
            continue;
        } else if skip_next {
            skip_next = false;
            continue;
        }
        let ent: Vec<u64> = line
            .split(" ")
            .map(|w| w.parse().expect("bad int"))
            .collect();
        maps_raw.last_mut().expect("empty").push(ent);
    }
    let mut maps: Vec<Vec<Range>> = Vec::new();
    for map in maps_raw {
        maps.push(Vec::new());
        for range_raw in map {
            match range_raw[..] {
                [out_start, in_start, size] => {
                    maps.last_mut().expect("empty").push(Range {
                        in_start,
                        out_start,
                        size,
                    });
                }
                _ => {
                    panic!("bad range");
                }
            }
        }
        maps.last_mut().expect("empty").sort();
    }
    (seeds, maps)
}
