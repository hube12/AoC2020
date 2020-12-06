use std::time::Instant;
use std::fs;
use std::collections::{HashSet, HashMap};


fn part1(questions:Vec<String>)->usize{
    let mut count:usize=0;
    let mut set:HashSet<char>=HashSet::new();
    for question in questions {
        if question.is_empty(){
            count+=set.len();
            set.clear();
            continue;
        }
        question.chars().for_each(|x| {
            set.insert(x) ;
        } )
    }
    if !set.is_empty(){
        count+=set.len();
        set.clear();
    }
    count
}

fn part2(questions:Vec<String>)->usize{
    let mut count:usize=0;
    let mut map:HashMap<char,usize>=HashMap::new();
    let mut number_person:usize=0;
    for question in questions {
        if question.is_empty(){
            for response in map.values(){
                if response==&number_person{
                    count+=1;
                }
            }
            map.clear();
            number_person=0;
            continue;
        }
        question.chars().for_each(|x| {
            *map.entry(x).or_insert(0)+=1;

        });
        number_person+=1;
    }
    if !map.is_empty(){
        for response in map.values(){
            if response==&number_person{
                count+=1;
            }
        }
        map.clear();
    }
    count
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