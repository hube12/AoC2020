use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use pcre2::bytes::Regex;

fn process_input(puzzle: Vec<&str>) -> (HashMap<&str, Vec<Vec<&str>>>, Vec<&str>) {
    let mut rules_map: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    assert!(!puzzle.is_empty());
    let mut it = puzzle.iter();
    loop {
        let cur = it.next().unwrap();
        if cur.is_empty() {
            break;
        }
        let mut it2 = cur.split(": ");
        let index = it2.next().to_owned().unwrap();
        let rule = it2.next().to_owned().unwrap();
        assert!(it2.next().is_none());

        let mut v: Vec<Vec<&str>> = vec![];
        let mut it3 = rule.split(" | ");
        while let Some(cur) = it3.next().to_owned() {
            v.push(cur.split(" ").collect());
        }
        rules_map.insert(index, v);
    }
    let mut texts = vec![];
    while let Some(cur) = it.next() {
        texts.push(cur.clone());
    }
    (rules_map, texts)
}

fn make_regex(rule: &str, rules_map: &HashMap<&str, Vec<Vec<&str>>>, part2: bool) -> String {
    if part2 {
        if rule == "8" {
            return "(".to_owned() + &make_regex("42", rules_map, part2) + "+" + ")";
        }
        if rule == "11" {
            return "(?P<token>".to_owned() + &make_regex("42", rules_map, part2) + "(?&token)*" + &make_regex("31", rules_map, part2) + ")";
        }
    }
    if !rules_map.contains_key(rule) {
        return rule.trim_matches('"').parse().unwrap();
    }
    let mut acc = vec![];
    for rules in rules_map.get(rule).unwrap() {
        let mut x = String::from("(");
        for rule in rules {
            x += &*make_regex(rule, rules_map, part2);
        }
        x += ")";
        acc.push(x)
    }
    return "(".to_owned() + &acc.join("|") + ")";
}


fn part1(puzzle: Vec<String>) -> usize {
    let (seq, texts) = process_input(puzzle.iter().map(|x| &**x).collect::<Vec<&str>>());
    let r = Regex::new(&*("^".to_owned() + &make_regex("0", &seq, false) + "$")).unwrap();
    let mut s = 0;
    for text in texts {
        s += if r.is_match(text.as_ref()).unwrap() { 1 } else { 0 }
    }
    s
}


fn part2(puzzle: Vec<String>) -> usize {
    let (seq, texts) = process_input(puzzle.iter().map(|x| &**x).collect::<Vec<&str>>());
    let r = Regex::new(&*("^".to_owned() + &make_regex("0", &seq, true) + "$")).unwrap();
    let mut s = 0;
    for text in texts {
        s += if r.is_match(text.as_ref()).unwrap() { 1 } else { 0 }
    }
    s
}


fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut puzzle: Vec<String> = vec![];
    for line in lines {
        puzzle.push(line.parse::<String>().expect("Ouf that's not a string !"));
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(puzzle.clone()));
    println!("Took {}ms", now.elapsed().as_millis());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(puzzle.clone()));
    println!("Took {}ms", now.elapsed().as_millis());
}