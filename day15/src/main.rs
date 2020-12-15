#![allow(unreachable_code)]

use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn solve(bound: usize, puzzle: Vec<u64>) -> usize {
    let mut map: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut last_index = 1;
    let mut last = 1;
    for el in puzzle.iter().enumerate() {
        map.insert(*el.1, ((el.0 + 1) as u64, u64::MAX));
        last_index = el.0 + 1;
        last = *el.1;
    }
    for i in last_index + 1..bound + 1 {
        let item = map.entry(last);
        match item {
            Entry::Occupied(ref entry) if entry.get().1 == u64::MAX => {
                last = 0;
            }
            Entry::Occupied(entry) => {
                let z = entry.get();
                last = (z.0 - z.1) as u64;
            }
            _ => {
                panic!("Not possible");
                last = 0;
            }
        }
        map.entry(last).and_modify(|e| *e = (i as u64, e.0)).or_insert((i as u64, u64::MAX));
    }
    last as usize
}

fn part1(puzzle: Vec<u64>) -> usize {
    solve(2020, puzzle)
}

fn part2(puzzle: Vec<u64>) -> usize {
    solve(30000000, puzzle)
}

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut puzzle: Vec<u64> = vec![];
    for line in lines {
        let str = line.parse::<String>().expect("Ouf that's not a string !");
        let it = str.split(",");
        it.for_each(|x| puzzle.push(x.parse::<u64>().unwrap()));
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