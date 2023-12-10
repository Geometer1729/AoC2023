use std::{collections::BTreeSet, fs::read_to_string};

pub fn a() {
    let (start, maze) = parse();
    let path = follow(start, &maze);
    println!("{}", path.len() / 2);
}

pub fn b() {
    let (start, maze) = parse();
    let path = follow(start, &maze);
    let mut inside = false;
    let mut count = 0;
    for (y, col) in maze.iter().enumerate() {
        for (x, dirs) in col.iter().enumerate() {
            let onpath = path.contains(&(x, y));
            if inside && !onpath {
                count += 1;
            }
            // imagine a point moving just to the right
            // of the center it crosses whenever
            // the symbol connects to the right
            if onpath && dirs[1] {
                inside ^= true;
            }
        }
    }
    println!("{}", count);
}

fn parse() -> ((usize, usize), Vec<Vec<[bool; 4]>>) {
    let mut start = (0, 0);
    let maze = read_to_string("input_10")
        .expect("no file or so")
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.char_indices()
                .map(|(pos, char)| match char {
                    // ccw from East/+x
                    '|' => [false, true, false, true],
                    '-' => [true, false, true, false],
                    'L' => [true, true, false, false],
                    'J' => [false, true, true, false],
                    '7' => [false, false, true, true],
                    'F' => [true, false, false, true],
                    '.' => [false; 4],
                    'S' => {
                        start = (line_num, pos);
                        [false, true, false, true]
                    }
                    c => panic!("bad char {}",c),
                })
                .collect()
        })
        .collect();
    (start, maze)
}

fn follow(start: (usize, usize), maze: &Vec<Vec<[bool; 4]>>) -> BTreeSet<(usize, usize)> {
    let mut dir = 3;
    let mut pos = start;
    let mut path = BTreeSet::new();
    loop {
        path.insert(pos);
        let (x, y) = pos;
        pos = match dir {
            0 => (x + 1, y),
            1 => (x, y - 1),
            2 => (x - 1, y),
            3 => (x, y + 1),
            x => {
                panic!("bad dir {}", x)
            }
        };
        if path.contains(&pos) {
            if pos != start {
            } else {
                break;
            }
        }
        let from = (dir + 2) % 4;
        let opts = (&maze)[pos.1][pos.0];
        for (ind, opt) in opts.iter().enumerate() {
            if ind != from && *opt {
                dir = ind;
            }
        }
    }
    path
}
