use std::fs;
use std::time::Instant;
use std::collections::{HashMap};

fn part1(numbers: Vec<u64>) -> u64 {
    let size: usize = 25;
    let mut map: HashMap<u64, u32> = HashMap::new();
    for i in 0..25 {
        for j in 0..25 {
            if i != j {
                let sum = numbers.get(i).unwrap() + numbers.get(j).unwrap();
                *map.entry(sum).or_insert(0) += 1
            }
        }
    }

    for i in size..numbers.len() {
        let cur = *numbers.get(i).unwrap();
        if !map.contains_key(&cur) {
            return cur;
        }
        let first = *numbers.get(i - size).unwrap();
        for j in i - size + 1..i {
            let sum = first + *numbers.get(j).unwrap();
            if !map.contains_key(&sum) {
                panic!("ERROR, key should be contained")
            } else {
                *map.entry(sum).or_default() -= 1;
                if *map.get(&sum).unwrap() == 0 {
                    map.remove(&sum);
                }
            }
            let new_sum = numbers.get(i).unwrap() + numbers.get(j).unwrap();
            *map.entry(new_sum).or_insert(0) += 1
        }
    }


    0
}

fn part2(numbers: Vec<u64>, target: u64) -> u64 {
    let mut seq: Vec<u64> = vec![];
    let mut sum: u64 = 0;
    for number in numbers {
        seq.push(number);
        sum += number;
        while sum > target {
            let first:u64 = seq.remove(0);
            sum -= first;
        }
        if sum == target && seq.len() > 1 {
            return seq.iter().min().unwrap() + seq.iter().max().unwrap();
        }
    }
    return 0;
}


fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut numbers: Vec<u64> = vec![];
    for line in lines {
        numbers.push(line.parse::<u64>().expect("Ouf that's not a number !"))
    }
    println!("Running part1");
    let now = Instant::now();
    let target = part1(numbers.clone());
    println!("Found {}", target);
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(numbers.clone(), target));
    println!("Took {}us", now.elapsed().as_micros());
}