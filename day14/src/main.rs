use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn process_mask(mask: &String, value: u64) -> u64 {
    let repr = format!("{:036b}", value);

    let mut res = String::new();
    for it in mask.chars().zip(repr.chars()) {
        let (m, r) = it;
        let r = match m {
            '1' => { '1' }
            '0' => { '0' }
            _ => { r }
        };
        res.push(r);
    }
    let res = u64::from_str_radix(&*res, 2).unwrap();
    res
}

fn part1(puzzle: Vec<String>) -> usize {
    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();
    for p in puzzle {
        if p.starts_with("mask = ") {
            mask = p.replace("mask = ", "");
        } else {
            let mut it = p.split("] = ");

            let address = it.next().unwrap().replace("mem[", "").parse::<u64>().unwrap();
            let value = it.next().unwrap().parse::<u64>().unwrap();
            map.insert(address, process_mask(&mask, value));
        }
    }
    let mut s: u64 = 0;
    map.retain(|_key, value| {
        s += *value;
        false
    });
    s as usize
}

fn process_mask2(mask: &String, value: u64) -> Vec<u64> {
    let repr = format!("{:036b}", value);
    let count = 1usize << mask.matches('X').count();
    let mut res: Vec<String> = vec![String::new(); count];
    for it in mask.chars().zip(repr.chars()) {
        let (m, r) = it;
        for re in res.iter_mut() {
            let r = match m {
                '1' => { '1' }
                '0' => { r }
                _ => {
                    'X'
                }
            };
            re.push(r);
        }
    }
    let mut fin: Vec<u64> = vec![];
    for (i, pair) in res.iter_mut().enumerate() {
        let number=format!("{:0width$b}", i,width=mask.matches('X').count());
        let mut temp =pair.clone();
        for el in number.chars(){
            temp=temp.replacen('X', &*el.to_string(), 1);
        }
        fin.push(u64::from_str_radix(&*temp, 2).unwrap());
    }
    fin
}

fn part2(puzzle: Vec<String>) -> usize {
    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();
    for p in puzzle {
        if p.starts_with("mask = ") {
            mask = p.replace("mask = ", "");
        } else {
            let mut it = p.split("] = ");

            let address = it.next().unwrap().replace("mem[", "").parse::<u64>().unwrap();
            let value = it.next().unwrap().parse::<u64>().unwrap();
            let addresses = process_mask2(&mask, address);
            for add in addresses {
                map.insert(add, value);
            }
        }
    }
    let mut s: u64 = 0;
    map.retain(|_key, value| {
        s += *value;
        false
    });
    s as usize
}

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut puzzle: Vec<String> = vec![];
    for line in lines {
        puzzle.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(puzzle.clone()));
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(puzzle.clone()));
    println!("Took {}ms", now.elapsed().as_millis());
}