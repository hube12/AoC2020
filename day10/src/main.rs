use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn part1(mut adapters: Vec<u64>) -> usize {
    adapters.sort();
    let mut map: HashMap<u8, usize> = HashMap::new();
    let mut last: u64 = 0;
    for adapter in adapters.iter() {
        let diff = (adapter - last) as u8;
        if diff > 3 || diff < 1 {
            eprintln!("Impossible {} {}", adapter, last);
            return 0;
        }
        *map.entry(diff).or_insert(0) += 1;
        last = *adapter;
    }
    *map.entry(3).or_insert(0) += 1;
    dbg!(&map);
    map.get(&1u8).unwrap() * map.get(&3u8).unwrap()
}

fn part2(mut adapters: Vec<u64>) -> usize {
    adapters.sort();
    adapters.insert(0, 0); // insert first voltage
    let nb_vertices: usize = adapters.len();
    let mut i: isize = (nb_vertices - 2) as isize;
    let mut path_counts: Vec<u64> = vec![0; nb_vertices];
    path_counts[nb_vertices - 1] = 1;
    while i >= 0 {
        let mut nb_paths = 0;
        for j in i + 1..nb_vertices as isize {
            if adapters[j as usize] - adapters[i as usize] > 3 {
                break;
            }
            nb_paths += path_counts[j as usize];
        }
        path_counts[i as usize] = nb_paths;
        i -= 1;
    }

    path_counts[0] as usize
}


fn part2bis(mut adapters: Vec<u64>) -> usize {
    adapters.sort();
    let total_length = adapters.len();
    let mut path_counts: Vec<usize> = vec![0; total_length];
    for i in 0..total_length {
        if adapters[i] - 0 > 3 {
            break;
        }
        path_counts[i] = 1;
    }
    for i in 0..total_length-1 {
        for j in i + 1..total_length {
            if adapters[j] - adapters[i] > 3 {
                break;
            }
            path_counts[j] += path_counts[i];
        }
    }
    *path_counts.last().unwrap()
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

    println!("Running part2 bis");
    let now = Instant::now();
    println!("Found {}", part2bis(adapters.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}