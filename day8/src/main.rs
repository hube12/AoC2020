use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn process_automaton(questions: Vec<String>, error:bool) -> isize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut i: i64 = 0;
    let mut acc: i64 = 0;
    while i < questions.len() as i64 && !set.contains(&(i as usize)) {
        let ins = questions.get(i as usize).expect("Missing instruction");
        if ins.starts_with("nop") {
            set.insert(i as usize);
            i += 1;
            continue;
        } else if ins.starts_with("acc") {
            let mut it = ins.split(" ");
            it.next();
            let number = it.next().expect("missing number");
            acc += number.parse::<i64>().expect("no error");
            set.insert(i as usize);
            i += 1;
        } else {
            set.insert(i as usize);
            let mut it = ins.split(" ");
            it.next();
            let number = it.next().expect("missing number");
            i += number.parse::<i64>().expect("no error");
        }
    }
    if i == questions.len() as i64 || !error { acc  as isize} else { -1 }
}
fn part1(questions: Vec<String>) -> isize {
    process_automaton(questions, false) as isize
}
fn part2(questions: Vec<String>) -> isize {
    process_automaton(questions, true) as isize
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
        let mut t = questions.clone();
        let line = t.get(i).expect("ee");
        if line.starts_with("jmp") {
            t[i] = line.replace("jmp", "nop");
        } else if line.starts_with("nop") {
            t[i] = line.replace("nop", "jmp");
        }
        let res = part2(t);
        if res != -1 {
            println!("Found {}", res);
        }
    }

    println!("Took {}ms", now.elapsed().as_millis());
}