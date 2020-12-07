use std::fs;
use std::time::Instant;

fn part1(questions:Vec<String>) ->usize{

}
fn part2(questions:Vec<String>)->usize{

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
    println!("Found {}", part2(questions.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}