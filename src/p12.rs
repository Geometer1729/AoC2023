use std::{cmp::min, collections::HashMap, fs::read_to_string};

use itertools::Itertools;

pub fn a() {
    let total: usize = read_to_string("input_12")
        .expect("no file or so")
        .lines()
        .map(|line| {
            let mut table = HashMap::new();
            let (springs, nums_raw) = line.split_once(' ').expect("no ' '");
            let nums: Vec<usize> = nums_raw
                .split(',')
                .map(|n| n.parse().expect("bad num"))
                .collect();
            let runs = to_runs(springs);
            solve(&mut table, None, runs[0], &runs[1..], nums[0], &nums[1..])
        })
        .sum();
    println!("{total}");
}

pub fn b() {
    let total: usize = read_to_string("input_12")
        .expect("no file or so")
        .lines()
        .map(|line| {
            let mut table = HashMap::new();
            let (springs_raw, nums_raw) = line.split_once(' ').expect("no ' '");
            let nums_raw: Vec<usize> = nums_raw
                .split(',')
                .map(|n| n.parse().expect("bad num"))
                .collect();
            let nums: Vec<usize> = nums_raw
                .iter()
                .cycle()
                .take(nums_raw.len() * 5)
                .map(|&x| x)
                .collect();
            let springs = [springs_raw].iter().cycle().take(5).join("?");
            let runs = to_runs(&springs);
            solve(&mut table, None, runs[0], &runs[1..], nums[0], &nums[1..])
        })
        .sum();
    println!("{total}");
}

type Run = (char, usize);

fn to_runs(w: &str) -> Vec<Run> {
    let mut chars = w.chars();
    let mut ret = Vec::new();
    let mut cur = (chars.next().expect("empty"), 1);
    for c in chars {
        if c == cur.0 {
            cur.1 += 1;
        } else {
            ret.push(cur);
            cur = (c, 1);
        }
    }
    ret.push(cur);
    ret
}

type MemoTable<'a> = HashMap<(Option<bool>, Run, &'a [Run], usize, &'a [usize]), usize>;

// This should really be split up and probably simplified
// there seem to be a lot of paterns especially around advancement
// ie checking if either list is long enough
fn solve<'a>(
    table: &mut MemoTable<'a>,
    next_must: Option<bool>,
    cur: Run,
    rest: &'a [Run],
    cur_num: usize,
    rest_nums: &'a [usize],
) -> usize {
    let key = (next_must, cur, rest, cur_num, rest_nums);
    match table.get(&key) {
        None => {
            let res = match cur {
                ('.', _) => {
                    if next_must == Some(true) {
                        0 // The next had to be a spring and wasn't
                    } else if rest.len() == 0 {
                        0 // ran out of springs with runs left
                    } else {
                        solve(table, None, rest[0], &rest[1..], cur_num, rest_nums)
                        // skip this run but remove any constraint on the next space
                    }
                }
                ('#', c) => {
                    if next_must == Some(false) {
                        0 // The next couldn't be a spring and was
                    } else if c > cur_num {
                        0 // to many springs for the next run
                    } else if c == cur_num {
                        // Curent run exactly meats num
                        if rest_nums.len() != 0 && rest.len() != 0 {
                            // skip one of each, next square can't be a spring
                            solve(
                                table,
                                Some(false),
                                rest[0],
                                &rest[1..],
                                rest_nums[0],
                                &rest_nums[1..],
                            )
                        } else if rest_nums.len() == 0 {
                            // No more nums just check for forced springs
                            ways_to_be_empty(rest)
                        } else {
                            // No more spaces but some runs left
                            0
                        }
                    } else {
                        // curent run is not quite long enough
                        // but may be extendable
                        if rest.len() == 0 {
                            0
                        } else {
                            solve(
                                table,
                                Some(true),
                                rest[0],
                                &rest[1..],
                                cur_num - c,
                                rest_nums,
                            )
                        }
                    }
                }
                ('?', c) => {
                    match next_must {
                        Some(true) => {
                            if c < cur_num {
                                // all must be true and more
                                if rest.len() == 0 {
                                    0
                                } else {
                                    solve(
                                        table,
                                        Some(true),
                                        rest[0],
                                        &rest[1..],
                                        cur_num - c,
                                        rest_nums,
                                    )
                                }
                            } else if c == cur_num {
                                // mystery springs used up exactly
                                if rest.len() != 0 && rest_nums.len() != 0 {
                                    // advance both
                                    // next can't be spring cause run just ended
                                    solve(
                                        table,
                                        Some(false),
                                        rest[0],
                                        &rest[1..],
                                        rest_nums[0],
                                        &rest_nums[1..],
                                    )
                                } else if rest_nums.len() == 0 {
                                    ways_to_be_empty(rest)
                                } else {
                                    // runs left but no space
                                    0
                                }
                            } else {
                                // run is more than long enoug
                                if rest_nums.len() == 0 {
                                    ways_to_be_empty(rest)
                                } else {
                                    // mystery run not over
                                    // but can't be a run imediately
                                    solve(
                                        table,
                                        Some(false),
                                        ('?', c - cur_num),
                                        rest,
                                        rest_nums[0],
                                        &rest_nums[1..],
                                    )
                                }
                            }
                        }
                        Some(false) => {
                            if c == 1 {
                                // mystery section all false cause it's only one space
                                if rest.len() == 0 {
                                    // ran out of space
                                    0
                                } else {
                                    // skip section and continue
                                    solve(table, None, rest[0], &rest[1..], cur_num, rest_nums)
                                }
                            } else {
                                // first space is false handle remaining section
                                solve(table, None, ('?', c - 1), rest, cur_num, rest_nums)
                            }
                        }
                        None => {
                            // The hard part
                            // Mystery section with no imediate forcing
                            let skipping = if rest.len() != 0 {
                                solve(table, None, rest[0], &rest[1..], cur_num, rest_nums)
                            } else {
                                0
                            };
                            let starting: usize = if rest.len() != 0 {
                                (1..=min(c, cur_num - 1))
                                    .map(|used| {
                                        solve(
                                            table,
                                            Some(true),
                                            rest[0],
                                            &rest[1..],
                                            cur_num - used,
                                            rest_nums,
                                        )
                                    })
                                    .sum()
                            } else {
                                0
                            };
                            let finishing = if cur_num > c {
                                // not enough space to finish
                                0
                            } else {
                                let ending = if rest.len() != 0 && rest_nums.len() != 0 {
                                    // advance both but can't start a new run
                                    solve(
                                        table,
                                        Some(false),
                                        rest[0],
                                        &rest[1..],
                                        rest_nums[0],
                                        &rest_nums[1..],
                                    )
                                } else if rest_nums.len() == 0 {
                                    ways_to_be_empty(rest)
                                } else {
                                    // no more space but still springs
                                    0
                                };
                                if cur_num == c {
                                    ending
                                } else {
                                    if rest_nums.len() == 0 {
                                        (c - cur_num + 1) * ways_to_be_empty(rest)
                                    } else {
                                        (1..=c - cur_num)
                                            .map(|leaving| {
                                                solve(
                                                    table,
                                                    Some(false),
                                                    ('?', leaving),
                                                    rest,
                                                    rest_nums[0],
                                                    &rest_nums[1..],
                                                )
                                            })
                                            .sum::<usize>()
                                            + ending
                                    }
                                }
                            };
                            skipping + starting + finishing
                        }
                    }
                }
                (char, _) => {
                    panic!("Bad char {char}")
                }
            };
            table.insert(key.clone(), res);
            res
        }
        Some(&res) => res,
    }
}

// bit silly as a function but keeps coming up
fn ways_to_be_empty(rest: &[Run]) -> usize {
    if rest.iter().any(|&(c, _)| c == '#') {
        0
    } else {
        1
    }
}
