use std::{cmp::min, collections::BTreeMap, fs::read_to_string};

pub fn a() {
    let txt = read_to_string("input_3").expect("no file or so");
    let lines = txt.lines().collect::<Vec<_>>();
    let total = lines.len();
    let sum: u64 = lines
        .iter()
        .enumerate()
        .map(|(ind, &line_raw)| {
            let runs = find_nums(line_raw);
            let line_len = line_raw.len();
            let start_line = ind.checked_sub(1).unwrap_or(0);
            let end_line = min(ind + 1, total - 1);
            runs.iter()
                .filter(|(num_start, num_end)| {
                    let start_pos = num_start.checked_sub(1).unwrap_or(0);
                    let end_pos = min(num_end + 1, line_len - 1);
                    (start_line..=end_line)
                        .map(|line| &lines[line][start_pos..=end_pos])
                        .any(|str| str.contains(|c: char| !c.is_digit(10) && c != '.'))
                })
                .map(|(start, end)| {
                    (line_raw[*start..=*end])
                        .parse::<u64>()
                        .expect("int didn't parse")
                })
                .sum::<u64>()
        })
        .sum::<u64>();
    println!("{}", sum);
}

pub fn b() {
    let txt = read_to_string("input_3").expect("no file or so");
    let lines = txt.lines().collect::<Vec<_>>();
    let total = lines.len();
    let mut gears: BTreeMap<(usize, usize), Vec<u64>> = BTreeMap::new();
    for (ind, line_raw) in lines.iter().enumerate() {
        let runs = find_nums(line_raw);
        let line_len = line_raw.len();
        let start_line = ind.checked_sub(1).unwrap_or(0);
        let end_line = min(ind + 1, total - 1);
        let gear_nums = runs
            .iter()
            .map(|(num_start, num_end)| {
                let start_pos = num_start.checked_sub(1).unwrap_or(0);
                let end_pos = min(num_end + 1, line_len - 1);
                let adjacent_text = (start_line..=end_line)
                    .map(|l| &lines[l][start_pos..=end_pos])
                    .collect::<Vec<_>>();
                (adjacent_text, start_pos, num_start, num_end)
            })
            .filter(|(adjacent_text, _, _, _)| adjacent_text.iter().any(|s| s.contains('*')))
            .map(|(adjacent_text, pos_start, start, end)| {
                let (rel_line, rel_pos) = adjacent_text
                    .iter()
                    .enumerate()
                    .map(|(line, &text)| text.find('*').map(|pos| (line, pos)))
                    .fold(None, |a, b| a.or(b))
                    .expect("found '*' but no '*'");
                let value = line_raw[*start..=min(*end, line_len - 1)]
                    .parse::<u64>()
                    .expect("int didn't parse");
                ((rel_pos + pos_start, rel_line + start_line), value)
            });
        for (pt, val) in gear_nums {
            let gear = gears.get_mut(&pt);
            match gear {
                None => {
                    gears.insert(pt, vec![val]);
                }
                Some(vec) => {
                    vec.push(val);
                }
            }
        }
    }
    println!(
        "{}",
        gears
            .values()
            .filter(|parts| parts.len() >= 2)
            .map(|parts| parts.iter().product::<u64>())
            .sum::<u64>()
    );
}

fn find_nums(line_raw: &str) -> Vec<(usize, usize)> {
    let mut line = line_raw.char_indices();
    let len = line_raw.len();
    let mut runs = Vec::new();
    loop {
        let start = match line.find(|c| c.1.is_digit(10)) {
            Some((ind, _)) => ind,
            None => {
                break;
            }
        };
        let end = match line.find(|c| !c.1.is_digit(10)) {
            Some((ind, _)) => ind - 1,
            None => len - 1,
        };
        runs.push((start, end));
    }
    runs
}
