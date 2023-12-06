use std::fs::read_to_string;


pub fn a(){
    let txt = read_to_string("input_4").expect("no file or so");
    let lines = txt
        .lines()
        .collect::<Vec<_>>();
    let mut total = 0;
    for line in  lines {
        let (card_have_raw,wining_raw) = line.split_once("|").expect("no |");
        let (_,have_raw) = card_have_raw.split_once(":").expect("no :");
        let wining : Vec<u64>  =
            wining_raw
            .trim_start()
            .trim_end()
            .split(" ")
            .filter(|w|w != &"")
            .map(|r|r.parse().expect("bad int"))
            .collect();
        let have =
            have_raw
            .trim_start()
            .split(" ")
            .filter(|w|w != &"")
            .map(|r|r.parse::<u64>().expect("bad int")) ;
        let matches = have
            .filter(|n|wining.contains(n))
            .count();
        let score : u64 =
            matches
            .checked_sub(1)
            .map(|n|1 << n)
            .unwrap_or(0);
        //dbg!(score);
        total += score;
    }
    println!("{}",total)
}


pub fn b(){
    let txt = read_to_string("input_4").expect("no file or so");
    let lines = txt
        .lines()
        .collect::<Vec<_>>();
    let mut scores = Vec::new();
    for line in  lines {
        let (card_have_raw,wining_raw) = line.split_once("|").expect("no |");
        let (_,have_raw) = card_have_raw.split_once(":").expect("no :");
        let wining : Vec<u64>  =
            wining_raw
            .trim_start()
            .trim_end()
            .split(" ")
            .filter(|w|w != &"")
            .map(|r|r.parse().expect("bad int"))
            .collect();
        let have =
            have_raw
            .trim_start()
            .split(" ")
            .filter(|w|w != &"")
            .map(|r|r.parse::<u64>().expect("bad int")) ;
        let score = have
            .filter(|n|wining.contains(n))
            .count();
        scores.push(score);
    }
    let mut total = 0;
    let mut mults = vec![1;scores.len()];
    for (ind,&score) in scores.iter().enumerate()
    {
        let mult = mults[ind];
        total += mult;
        for i in ind+1..ind+(score as usize)+1 {
            mults[i] += mult;
        }
    }
    println!("{}",total)
}
