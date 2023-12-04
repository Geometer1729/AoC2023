use std::{fs::read_to_string, collections::BTreeMap};

pub fn p1_a(){
    let offset = "0".as_bytes()[0];
    let mut sum : u64 = 0;
    for line in read_to_string("input_1").unwrap().lines() {
        let bs = line.as_bytes();
        let c1 :u64 =
            (bs[line
                .find(|c:char|c.is_digit(10))
                .expect("no digits")
                ]
            - offset
            ) as u64;
        let c2 :u64 =
            (bs[line
                .rfind(|c:char|c.is_digit(10))
                .expect("no digits")
                ]
            - offset
            ) as u64;
        sum +=c1*10+c2;
    }
    println!("{}",sum);
}

pub fn p1_b(){
    let offset = "0".as_bytes()[0];
    let mut sum : u64 = 0;
    let names: BTreeMap<&str,u64> = BTreeMap::from([
       ("zero",0),
       ("one",1),
       ("two",2),
       ("three",3),
       ("four",4),
       ("five",5),
       ("six",6),
       ("seven",7),
       ("eight",8),
       ("nine",9),
    ]);

    for line in read_to_string("input_1").unwrap().lines() {
        let bs = line.as_bytes();
        let mut ls = BTreeMap::new();
        let dl = line
                .find(|c:char|c.is_digit(10));
        match dl{
            Some(ind) => {ls.insert(ind,(bs[ind]-offset) as u64);},
            None => {},
        }
        for (name,val) in &names {
            match line.find(name) {
                Some(ind) => {ls.insert(ind,*val);},
                None => {},
            }
        }
        let (_,&cl) = ls.first_key_value().expect("ls empty");

        // TODO this is kinda cringe repeditive
        let mut rs = BTreeMap::new();
        let dr = line
                .rfind(|c:char|c.is_digit(10));
        match dr{
            Some(ind) => {rs.insert(ind,(bs[ind]-offset) as u64);},
            None => {},
        }
        for (name,val) in &names {
            match line.rfind(name) {
                Some(ind) => {rs.insert(ind,*val);},
                None => {},
            }
        }
        let (_,&cr) = rs.last_key_value().expect("rs empty");

        sum +=cl*10+cr;
    }
    println!("{}",sum);
}

