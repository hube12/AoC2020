#![allow(unreachable_code)]

use std::fs;
use std::time::Instant;

fn solve(bound: usize, puzzle: Vec<u64>) -> usize {
    let mut last_index = 1;
    let mut last = 1;
    let mut v = vec![(u32::MAX, u32::MAX); bound];
    for el in puzzle.iter().enumerate() {
        v[*el.1 as usize] = ((el.0 + 1) as u32, u32::MAX);
        last_index = el.0 + 1;
        last = *el.1;
    }
    for i in last_index + 1..bound + 1 {
        if v[last as usize].1 == u32::MAX {
            last = 0
        } else {
            last = i as u64 - v[last as usize].1 as u64 - 1;
        }
        v[last as usize] = (i as u32, v[last as usize].0);
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