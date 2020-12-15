use std::fs;
use std::time::Instant;
use std::collections::HashMap;

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
        if !map.contains_key(&last) {
            last = 0;
        } else {
            let turn = map.get(&last).unwrap();
            if turn.1 == u64::MAX {
                last = 0;
            } else {
                last = (turn.0 - turn.1) as u64;
            }
        }
        let z = map.get(&last).unwrap_or(&(u64::MAX, u64::MAX)).0;
        map.insert(last, (i as u64, z));
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