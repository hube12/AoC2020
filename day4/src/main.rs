#![allow(dead_code, unused_variables, unused_imports)]
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use regex::Regex;

const FIELDS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn check_fields(storage: &HashMap<String, String>, strong_verification: bool) -> bool {
    if !storage.contains_key("byr") {
        return false;
    }
    if strong_verification {
        let parse = storage.get("byr").unwrap().parse::<u32>();
        if parse.is_err() || *parse.as_ref().unwrap() < 1920 || *parse.as_ref().unwrap() > 2002 {
            return false;
        }
    }

    if !storage.contains_key("iyr") {
        return false;
    }
    if strong_verification {
        let parse = storage.get("iyr").unwrap().parse::<u32>();
        if parse.is_err() || *parse.as_ref().unwrap() < 2010 || *parse.as_ref().unwrap() > 2020 {
            return false;
        }
    }

    if !storage.contains_key("eyr") {
        return false;
    }
    if strong_verification {
        let parse = storage.get("eyr").unwrap().parse::<u32>();
        if parse.is_err() || *parse.as_ref().unwrap() < 2020 || *parse.as_ref().unwrap() > 2030 {
            return false;
        }
    }

    if !storage.contains_key("hgt") {
        return false;
    }
    if strong_verification {
        let height = storage.get("hgt").unwrap();
        if height.ends_with("in") {
            let parse = height.trim_end_matches("in").parse::<u32>();
            if parse.is_err() || *parse.as_ref().unwrap() < 59 || *parse.as_ref().unwrap() > 76 {
                return false;
            }
        } else if height.ends_with("cm") {
            let parse = height.trim_end_matches("cm").parse::<u32>();
            if parse.is_err() || *parse.as_ref().unwrap() < 150 || *parse.as_ref().unwrap() > 193 {
                return false;
            }
        } else { return false; }
    }

    if !storage.contains_key("hcl") {
        return false;
    }
    if strong_verification {
        let hair = storage.get("hcl").unwrap();
        let re = Regex::new(r"^#[\da-f]{6}$").unwrap();
        if !re.is_match(hair) {
            return false;
        }
    }

    if !storage.contains_key("ecl") {
        return false;
    }
    if strong_verification {
        let eye = storage.get("ecl").unwrap();
        if !ECL.contains(&&**eye) {
            return false;
        }
    }
    if !storage.contains_key("pid") {
        return false;
    }
    if strong_verification {
        let pid = storage.get("pid").unwrap();
        let re = Regex::new(r"^\d{9}$").unwrap();
        if !re.is_match(pid) {
            return false;
        }
    }

    return true;
}

fn part1(passports: Vec<String>) -> usize {
    let mut valid: usize = 0;
    let mut storage: HashMap<String, String> = HashMap::new();
    for passport in passports {
        if passport.is_empty() {
            if check_fields(&storage, false) {
                valid += 1;
            }
            storage.clear();
            continue;
        }
        let fields = passport.split(" ");
        for field in fields {
            let mut it = field.split(":");
            let key = it.next().expect("missing key");
            let value = it.next().expect("missing value");
            storage.insert(key.parse().unwrap(), value.parse().unwrap());
        }
    }
    if !storage.is_empty() {
        if check_fields(&storage, false) {
            valid += 1;
        }
        storage.clear();
    }
    valid
}

fn part2(passports: Vec<String>) -> usize {
    let mut valid: usize = 0;
    let mut storage: HashMap<String, String> = HashMap::new();
    for passport in passports {
        if passport.is_empty() {
            if check_fields(&storage, true) {
                valid += 1;
            }
            storage.clear();
            continue;
        }
        let fields = passport.split(" ");
        for field in fields {
            let mut it = field.split(":");
            let key = it.next().expect("missing key");
            let value = it.next().expect("missing value");
            storage.insert(key.parse().unwrap(), value.parse().unwrap());
        }
    }
    if !storage.is_empty() {
        if check_fields(&storage, true) {
            valid += 1;
        }
        storage.clear();
    }
    valid
}

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut passports: Vec<String> = vec![];
    for line in lines {
        passports.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(passports.clone()));
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(passports.clone()));
    println!("Took {}ms", now.elapsed().as_millis());
}
