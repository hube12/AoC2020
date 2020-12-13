use std::fs;
use std::time::Instant;
use std::ops::Neg;

fn part1(puzzle:Vec<String>) -> i32 {
    let timestamp=puzzle.get(0).unwrap().parse::<u64>().unwrap();
    let services=puzzle.get(1).unwrap().split(",").filter(|&x| x!="x").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut times =Vec::new();
    let mut min =i32::MAX;
    let mut min_index =0;
    for service in services{
        let mut time =(timestamp% (service as u64)) as i32 - service as i32;
        time=time.neg();
        times.push(time);
        if time<min{
            min=time;
            min_index=service;
        }
    }
    (min_index * min as u32) as i32
}
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part2(puzzle:Vec<String>)->usize{
    let mut it =puzzle.get(1).unwrap().split(",");
    let mut residues:Vec<i64> =vec![];
    let mut modulii:Vec<i64>  =vec![];

    let mut i=0;
    while let Some(cur )= it.next() {
        if cur!="x"{
            let number=cur.parse::<u32>().unwrap();

            modulii.push(number as i64);
            residues.push(((number-(i%number))%number) as i64);
        }
        i+=1;
    }
    chinese_remainder(&residues, &modulii).unwrap() as usize
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
    println!("Took {}us", now.elapsed().as_micros());
}