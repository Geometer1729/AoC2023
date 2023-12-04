use std::{fs::read_to_string, collections::BTreeMap, cmp::min};

pub fn a() {
    let txt = read_to_string("input_3").expect("no file or so");
    let lines = txt
        .lines()
        .collect::<Vec<_>>();
    let total = lines.len();
    let mut parts:Vec<u64> = Vec::new();
    for (ind,line_raw) in  lines.iter().enumerate() {
        let mut line = line_raw.chars();
        let len = line_raw.len();
        let mut pos = 0;
        let mut runs = Vec::new();
        loop {
            let start = match line.position(|c|c.is_digit(10)) {
                Some(ind) => {ind},
                None => {break;},
            };
            let end = match line.position(|c|!c.is_digit(10)) {
                Some(ind) => ind,
                None => len-pos-1,
            };
            runs.push((pos+start,pos+start+end));
            pos += start+end+2;
        }
        let line_start = ind.checked_sub(1).unwrap_or(0);
        let line_end = min(ind+2,total);

        for (start,end) in runs {
            let pos_start = start.checked_sub(1).unwrap_or(0);
            let pos_end = min(end+2,len);
            let value = line_raw[start..min(end+1,len)]
                .parse()
                .expect("int didn't parse");
            //dbg!(pos_start,pos_end,line_start,line_end);
            let slices = (line_start..line_end)
                .map(|l|& lines[l][pos_start..pos_end])
                .collect::<Vec<_>>();
            if slices.iter().any(|s|s.contains(|c:char|!c.is_digit(10) && c != '.'))
            {
                parts.push(value);
            }
        }
    }
    println!("{}",parts.iter().sum::<u64>());
}


pub fn b() {
    let txt = read_to_string("input_3").expect("no file or so");
    let lines = txt
        .lines()
        .collect::<Vec<_>>();
    let total = lines.len();
    let mut gears:BTreeMap<(usize,usize),Vec<u64>> = BTreeMap::new();
    for (ind,line_raw) in  lines.iter().enumerate() {
        let mut line = line_raw.chars();
        let len = line_raw.len();
        let mut pos = 0;
        let mut runs = Vec::new();
        loop {
            let start = match line.position(|c|c.is_digit(10)) {
                Some(ind) => {ind},
                None => {break;},
            };
            let end = match line.position(|c|!c.is_digit(10)) {
                Some(ind) => ind,
                None => len-pos-1,
            };
            runs.push((pos+start,pos+start+end));
            pos += start+end+2;
        }
        let line_start = ind.checked_sub(1).unwrap_or(0);
        let line_end = min(ind+2,total);
        for (start,end) in runs {
            let pos_start = start.checked_sub(1).unwrap_or(0);
            let pos_end = min(end+2,len);
            let value = line_raw[start..min(end+1,len)]
                .parse()
                .expect("int didn't parse");
            let slices = (line_start..line_end)
                .map(|l|& lines[l][pos_start..pos_end])
                .collect::<Vec<_>>();
            if slices.iter().any(|s|s.contains('*'))
            {
                let (rel_line,rel_pos) = slices
                    .iter()
                    .enumerate()
                    .map(|(l,&s)|s.find('*').map(|p|(l,p)))
                    .fold(None,|a,b|a.or(b))
                    .expect("no *?");
                let pt = (rel_pos+pos_start,rel_line+line_start);

                let gear = gears.get_mut(&pt);
                match gear {
                    None => { gears.insert(pt,vec![value]);},
                    Some(vec) => { vec.push(value); },
                }
            }
        }
    }
    dbg!(gears.clone());
    println!("{}",gears
             .values()
             .filter(|parts|parts.len() >= 2)
             .map(|parts|parts
                        .iter()
                        .product::<u64>())
             .sum::<u64>()
             );
}
