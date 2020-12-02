use std::fs;
use std::time::Instant;


fn validate_passwords_part1(lines:Vec<String>) ->usize{
    let mut count_valid:usize=0;
    for line in lines {
       let mut it =line.split(":");
        let pattern= it.next().expect("You are missing something ;)");
        let password= it.next().expect("You are missing something ;)").trim_start();
        assert_eq!(it.next(), None);
        let mut it =pattern.split_whitespace();
        let range= it.next().expect("You are missing something ;)");
        let letters= it.next().expect("You are missing something ;)");
        assert_eq!(letters.len(), 1);
        let letter=letters.chars().next().expect("Missing a char");
        let  it:Vec<usize> =range.split("-").map(|x| x.parse::<usize>().expect("Oof not a number")).collect();
        assert_eq!(it.len(), 2);
        let (min,max)=(*it.get(0).expect("Missing min"),*it.get(1).expect("Missing max"));
        let matches=password.matches(letter).count();
        if min<=matches && matches<=max {
            count_valid+=1;
        }
    }
    count_valid
}

fn validate_passwords_part2(lines:Vec<String>) ->usize{
    let mut count_valid:usize=0;
    for line in lines {
        let mut it =line.split(":");
        let pattern= it.next().expect("You are missing something ;)");
        let password= it.next().expect("You are missing something ;)").trim_start();
        assert_eq!(it.next(), None);
        let mut it =pattern.split_whitespace();
        let range= it.next().expect("You are missing something ;)");
        let letters= it.next().expect("You are missing something ;)");
        assert_eq!(letters.len(), 1);
        let letter=letters.chars().next().expect("Missing a char");
        let it:Vec<usize> =range.split("-").map(|x| x.parse::<usize>().expect("Oof not a number")).collect();
        assert_eq!(it.len(), 2);
        let (pos1,pos2)=(*it.get(0).expect("Missing min"),*it.get(1).expect("Missing max"));
        if pos1>password.len() || pos2>password.len(){
            continue
        }

        let char1: char =password.chars().nth((pos1-1) as usize).expect("Missing pos1");
        let char2:char=password.chars().nth((pos2-1) as usize).expect("Missing pos1");

        if (char1==letter || char2==letter) && (char1!=char2){
            count_valid+=1;
        }

    }
    count_valid
}



fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut passwords: Vec<String> = vec![];
    for line in lines {
        passwords.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }

    println!("Running part1");
    let now = Instant::now();
    println!("{}", validate_passwords_part1(passwords.clone()));
    println!("Took {}ms", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("{}", validate_passwords_part2(passwords.clone()));
    println!("Took {}ms", now.elapsed().as_micros());
}
