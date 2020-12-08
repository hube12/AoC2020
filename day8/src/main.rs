use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn part1(questions: Vec<String>) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut i: usize = 0;
    let mut acc: i64 = 0;
    while i < questions.len() && !set.contains(&i) {
        let ins = questions.get(i).expect("Missing instruction");
        if ins.starts_with("nop") {
            set.insert(i);
            i += 1;
            continue;
        } else if ins.starts_with("acc") {
            let mut it = ins.split(" ");
            it.next();
            let number = it.next().expect("missing number");
            if number.starts_with("+") {
                acc += number.trim_start_matches("+").parse::<i64>().expect("no error");
            } else {
                acc -= number.trim_start_matches("-").parse::<i64>().expect("no error");
            }
            set.insert(i);
            i += 1;
        } else {
            set.insert(i);
            let mut it = ins.split(" ");
            it.next();
            let number = it.next().expect("missing number");
            if number.starts_with("+") {
                i += number.trim_start_matches("+").parse::<usize>().expect("no error");
            } else {
                i -= number.trim_start_matches("-").parse::<usize>().expect("no error");
            }
        }
    }
    acc as usize
}

fn part2(questions: Vec<String>) -> isize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut i: usize = 0;
    let mut acc: i64 = 0;
    while i < questions.len() {
        let ins = questions.get(i).expect("Missing instruction");
        if ins.starts_with("nop") {
            set.insert(i);
            i += 1;
            continue;
        } else if ins.starts_with("acc") {
            let mut it = ins.split(" ");
            it.next();
            let number = it.next().expect("missing number");
            if number.starts_with("+") {
                acc += number.trim_start_matches("+").parse::<i64>().expect("no error");
            } else {
                acc -= number.trim_start_matches("-").parse::<i64>().expect("no error");
            }
            set.insert(i);
            i += 1;
        } else {
            set.insert(i);
            let mut it = ins.split(" ");
            it.next();
            let number = it.next().expect("missing number");
            let mut j=i;
            if number.starts_with("+") {
                j += number.trim_start_matches("+").parse::<usize>().expect("no error");
            } else {
                j -= number.trim_start_matches("-").parse::<usize>().expect("no error");
            }

            i=j;
        }
        if set.contains(&i){
            return -1;
        }

    }
    acc as isize
}

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut questions: Vec<String> = vec![];
    for line in lines {
        questions.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(questions.clone()));
    println!("Took {}us", now.elapsed().as_micros());
    println!("Running part2");
    let now = Instant::now();
    for i in 0..questions.len() {
        let mut t =questions.clone();
        let line=t.get(i).expect("ee");
        if line.starts_with("jmp"){
            t[i]=line.replace("jmp","nop");
        }else if line.starts_with("nop") {
            t[i]=line.replace("nop","jmp");
        }
        let res=part2(t);
        if res!=-1{
            println!("Found {}", res);
        }

    }

    println!("Took {}us", now.elapsed().as_micros());

}