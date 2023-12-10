use std::{collections::BTreeMap, fs::read_to_string};

pub fn a() {
    let sum = read_to_string("input_1")
        .expect("no file")
        .lines()
        .map(|line| {
            let ld = line
                .chars()
                .find(|&c: &char| c.is_digit(10))
                .expect("no digits");
            let rd = line
                .chars()
                .rfind(|&c: &char| c.is_digit(10))
                .expect("no digits");
            ((ld as u8 - '0' as u8) * 10 + (rd as u8 - '0' as u8)) as u64
        })
        .sum::<u64>();
    println!("{}", sum);
}

pub fn b() {
    let names: BTreeMap<&str, u64> = BTreeMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let sum: u64 = read_to_string("input_1")
        .expect("no file")
        .lines()
        .map(|line| {
            let mut ls = BTreeMap::new();
            let (l_ind, l_dig) = line
                .char_indices()
                .find(|&(_, c)| c.is_digit(10))
                .expect("no digits");
            ls.insert(l_ind, (l_dig as u8 - '0' as u8) as u64);

            let mut rs = BTreeMap::new();
            let (r_ind, r_dig) = line
                .char_indices()
                .rfind(|&(_, c)| c.is_digit(10))
                .expect("no digits");
            rs.insert(r_ind, (r_dig as u8 - '0' as u8) as u64);

            for (name, &val) in &names {
                line.find(name).map(|ind| {
                    ls.insert(ind, val);
                });
                line.rfind(name).map(|ind| {
                    rs.insert(ind, val);
                });
            }
            let &cl = ls.first_key_value().expect("ls empty").1;
            let &cr = rs.last_key_value().expect("rs empty").1;
            10 * cl + cr
        })
        .sum();
    println!("{}", sum);
}
