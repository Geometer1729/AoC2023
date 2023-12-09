use std::fs::read_to_string;

pub fn a() {
    let ents = parse();
    let edges = ents.iter().map(make_edge);
    let total: i64 = ents
        .iter()
        .zip(edges)
        .map(|(ent, edge)| {
            let next_ind = ent.len();
            edge.iter()
                .enumerate()
                .map(|(i, v)| choose(next_ind as i64, i as i64) * v)
                .sum::<i64>()
        })
        .sum();
    println!("{}", total);
}

pub fn b() {
    let ents = parse();
    let edges = ents.iter().map(make_edge);
    let total: i64 = edges
        .map(|edge| {
            edge.iter()
                .enumerate()
                .map(|(i, v)| (1 - 2 * (i as i64 % 2)) * v)
                .sum::<i64>()
        })
        .sum();
    println!("{}", total);
}

fn parse() -> Vec<Vec<i64>> {
    read_to_string("input_9")
        .expect("no file or so")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse().expect("bad int"))
                .collect()
        })
        .collect()
}

fn make_edge(ent: &Vec<i64>) -> Vec<i64> {
    let mut edge: Vec<i64> = vec![ent[0]];
    for i in 1.. {
        let vals = (0_i64..=i)
            .map(
                |ind| ent[ind as usize] * choose(i, ind) * (1 - 2 * ((i + ind) % 2)), // -1^(i+ind)
            )
            .collect::<Vec<_>>();
        let val = vals.iter().sum();
        edge.push(val);
        if val == 0 && edge[edge.len() - 2] == 0 {
            break;
        }
    }
    edge
}


fn choose(a: i64, b: i64) -> i64 {
    if b == 0 {
        1
    } else if b < a / 2 {
        choose(a, a - b)
    } else {
        (b + 1..=a).zip(1..).fold(1, |p, (x, y)| (p * x) / y)
    }
}
