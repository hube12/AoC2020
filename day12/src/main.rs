use std::fs;
use std::time::Instant;

fn part1(mut adapters: Vec<u64>) -> usize {
0
}
fn part2(mut adapters: Vec<u64>) -> usize {
0
}

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut adapters: Vec<u64> = vec![];
    for line in lines {
        adapters.push(line.parse::<u64>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(adapters.clone()));
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(adapters.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}