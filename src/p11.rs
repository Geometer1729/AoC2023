use std::fs::read_to_string;

pub fn a() {
    let (mut xs, ys) = parse();
    xs.sort();
    let total = count(xs, 2) + count(ys, 2);
    println!("{total}");
}

pub fn b() {
    let (mut xs, ys) = parse();
    xs.sort();
    let total = count(xs, 1_000_000) + count(ys, 1_000_000);
    println!("{total}");
}

fn parse() -> (Vec<usize>, Vec<usize>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    read_to_string("input_11")
        .expect("no file or so")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| (x, y))
                .for_each(|(x, y)| {
                    xs.push(x);
                    ys.push(y);
                })
        });
    // I want to do this with something like
    // maps instead of for_each and .kmerge().unzip()
    // but kmerge has a bunch of sorting overhead
    // I can't find a simple fold_chain() type thing
    (xs, ys)
}

fn count(v: Vec<usize>, scale: usize) -> i64 {
    let len = v.len() as i64;
    let mut expansion = 0;
    let mut last = v[0];
    v.iter()
        .map(|&c| {
            if c - last > 1 {
                expansion += (c - last - 1) * (scale - 1);
            }
            last = c;
            c + expansion
        })
        .enumerate()
        .map(|(ind, c)| (2 * ind as i64 + 1 - len) * c as i64)
        .sum()
}
