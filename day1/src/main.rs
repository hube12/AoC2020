use std::fs;
use std::cmp::min;
use std::process::exit;
use std::time::Instant;
use std::collections::BTreeSet;

fn optimized_solution_part1(numbers: Vec<u16>) -> () {
    let (mut low, high): (Vec<u16>, Vec<u16>) = numbers.iter().partition(|&&x| x <= (2020 / 2));
    let mut n: usize = min(low.len(), high.len());
    if n == 0 {
        assert!(low.len() >= 2);
        let last_low: u16 = *low.last().expect("Something was wrong, there should be an index here");
        low.pop();
        let last_last_low: u16 = *low.last().expect("Something was wrong, there should be an index here");
        assert_eq!(last_last_low, last_low);
        assert_eq!(last_last_low, 1024);
        println!("Found 1024 twice ! {}", last_last_low * last_low);
    }
    let mut last_index: usize = 0;
    while n > 0 {
        let last_low: u16 = *low.last().expect("Something was wrong, there should be an index here");
        low.pop();
        for i in last_index..high.len() {
            let current_high: u16 = *high.get(i).expect("Something was wrong, there should be an index here");
            if last_low + current_high == 2020 {
                println!("Found {} and {} which makes {}", last_low, current_high, last_low * current_high);
                return;
            }
            if last_low + current_high > 2020 {
                // we know that this low is not that good so we can discard it
                break;
            } else {
                last_index = i;
            }
        }
        n -= 1;
    }
    println!("Found no solution ;(");
}


fn optimized_solution_part1_hashset(numbers: Vec<u16>) -> () {
    let mut s:BTreeSet<u16>=BTreeSet::new();
    for i in 0..numbers.len() {
        let current=*numbers.get(i).expect("Something was wrong, there should be an index here");
        let diff=2020-current;
        if s.contains(&diff){
            println!("Found {} and {} which makes {}",diff,current,diff*current);
            return;
        }
        s.insert(current);
    }
    println!("Found no solution ;(");
}

fn dumb_solution_part1(numbers: Vec<u16>) {
    for i in 0..numbers.len() {
        let outer: u16 = *numbers.get(i).expect("Something was wrong, there should be an index here");
        for j in i..numbers.len() {
            let inner: u16 = *numbers.get(j).expect("Something was wrong, there should be an index here");
            if inner + outer == 2020 {
                println!("Found {} and {} which makes {}", outer, inner, outer * inner);
                return;
            }
        }
    }
    println!("Found no solution ;(");
}


fn dumb_solution_part2(numbers: Vec<u16>) {
    for i in 0..numbers.len() {
        let n1: u16 = *numbers.get(i).expect("Something was wrong, there should be an index here");
        for j in i..numbers.len() {
            let n2: u16 = *numbers.get(j).expect("Something was wrong, there should be an index here");
            for k in j..numbers.len() {
                let n3: u16 = *numbers.get(k).expect("Something was wrong, there should be an index here");
                if n1 + n2 + n3 == 2020 {
                    println!("Found {}, {} and {} which makes {}", n1, n2, n3, (n1 as u64).wrapping_mul(n2 as u64).wrapping_mul(n3 as u64));
                    return;
                }
            }
        }
    }
    println!("Found no solution ;(");
}


fn optimized_solution_part2(numbers: Vec<u16>) -> () {
    let n=numbers.len();
    for i in 0..n - 2 {
        let a=*numbers.get(i).expect("Something was wrong, there should be an index here");
        let mut start = i + 1;
        let mut end = n - 1;
        while start < end {
            let b = *numbers.get(start).expect("Something was wrong, there should be an index here");
            let c = *numbers.get(end).expect("Something was wrong, there should be an index here");
            if a + b + c == 0 {
                println!("Found {}, {} and {} which makes {}", a,b,c, (a as u64).wrapping_mul(b as u64).wrapping_mul(c as u64));
                start = start + 1;
                end = end - 1;
                return;
            }
            else if a + b + c > 0 {
                end = end - 1;
            }
            else{
                start = start + 1;
            }

        }
    }
    println!("Found no solution ;(");
}


fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let mut lines = input.lines();
    let mut numbers: Vec<u16> = vec![];
    for line in lines {
        numbers.push(line.parse::<u16>().expect("Ouf that's not a number !"))
    }
    // this is the key part
    numbers.sort();



    println!("Running dumb solution part1");
    let now = Instant::now();
    dumb_solution_part1(numbers.clone());
    println!("Took {}ms", now.elapsed().as_micros());

    println!("Running optimized solution part1");
    let now = Instant::now();
    optimized_solution_part1(numbers.clone());
    println!("Took {}ms", now.elapsed().as_micros());

    println!("Running optimized solution part1 hashset");
    let now = Instant::now();
    optimized_solution_part1_hashset(numbers.clone());
    println!("Took {}ms", now.elapsed().as_micros());

    println!("Running dumb solution part2");
    let now = Instant::now();
    dumb_solution_part2(numbers.clone());
    println!("Took {}ms", now.elapsed().as_micros());

    println!("Running optimized solution part2");
    let now = Instant::now();
    dumb_solution_part2(numbers.clone());
    println!("Took {}ms", now.elapsed().as_micros());
}
