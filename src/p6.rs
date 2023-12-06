use std::fs::read_to_string;

pub fn a(){
    let txt = read_to_string("input_6").expect("no file or so");
    let mut lines = txt.lines();
    let times = parse_line(lines.next().expect("file too short"));
    let records = parse_line(lines.next().expect("file too short"));
    let mut total = 1;
    for (&time,record) in times.iter().zip(records.iter()) {
        let slop_4x = time*time-4*record;
        let max_double_dist_to_half = (slop_4x as f64).sqrt();
        let low  = (((time as f64) - max_double_dist_to_half)/2.0 +1.0).floor() as u64;
        let high = (((time as f64) + max_double_dist_to_half)/2.0 -1.0).ceil() as u64;
        total *= high-low + 1;
    }
    println!("{}",total);
}


pub fn b(){
    let txt = read_to_string("input_6").expect("no file or so");
    let mut lines = txt.lines();
    let time = parse_line_b(lines.next().expect("file too short"));
    let record = parse_line_b(lines.next().expect("file too short"));
    let slop_4x = time*time-4*record;
    let max_double_dist_to_half = (slop_4x as f64).sqrt();
    let low  = (((time as f64) - max_double_dist_to_half)/2.0 +1.0).floor() as u64;
    let high = (((time as f64) + max_double_dist_to_half)/2.0 -1.0).ceil() as u64;
    //dbg!(max_double_dist_to_half,low,high);
    let total = high-low + 1;
    println!("{}",total);
}

fn parse_line(w:&str) -> Vec<u64> {
    w
        .split_once(':')
        .expect("no :")
        .1
        .trim_start()
        .split(" ")
        .filter(|w|w != &"")
        .map(|w|w.parse().expect("bad int"))
        .collect()

}


fn parse_line_b(w:&str) -> u64 {
    w
        .split_once(':')
        .expect("no :")
        .1
        .trim_start()
        .split(" ")
        .filter(|w|w != &"")
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .expect("bad int")
}
