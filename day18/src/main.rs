use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn reverse_polish_notation<'a>(tokens: Vec<&'a str>, precedence: &'a HashMap<&str, i32>) -> Vec<&'a str> {
    let mut res:Vec<&str> = vec![];
    let mut operator:Vec<&str> = vec![];
    for token in tokens {
        match token {
            "+" | "*" => {
                while operator.len() > 0 {
                    let op:&str = operator.pop().unwrap();
                    if op != "(" && precedence.get(op).unwrap() >= precedence.get(token).unwrap() {
                        res.push(op);
                    } else {
                        operator.push(op);
                        break;
                    }
                }
                operator.push(token);
            }
            "(" => {
                operator.push(token);
            }
            ")" => {
                loop {
                    let op:&str = operator.pop().expect("No Matching parenthesis");
                    if op == "(" {
                        break;
                    }
                    res.push(op);
                }
            }
            _ => {
                res.push(&token);
            }
        }

    }
    operator.reverse();
    for op in &operator {
        res.push(op);
    }
    res
}

fn eval(expression:Vec<&str>)->usize{
    let mut stack =vec![];
    for token in expression{
        match token {
            "+"=>{
                let res=stack.pop().unwrap()+stack.pop().unwrap();
                stack.push(res)
            }
            "*"=>{
                let res=stack.pop().unwrap()*stack.pop().unwrap();
                stack.push(res)
            }
            _=>{
                stack.push(token.parse::<usize>().unwrap())
            }
        }
    }
    stack.pop().unwrap()
}
fn run(puzzle:Vec<String>,precedence:HashMap<&str, i32>)-> usize{
    let mut s =0;
    for mut line in puzzle{
        line=line.replace("(","( ");
        line=line.replace(")"," )");
        let l=line.split(" ").collect::<Vec<&str>>();
        s+=eval(reverse_polish_notation(l,&precedence));
    }
    s
}
fn part1(puzzle: Vec<String>) -> usize {
    let mut precedence:HashMap<&str,i32>=HashMap::new();
    precedence.insert("+",1);
    precedence.insert("*",1);
    run(puzzle,precedence)

}

fn part2(puzzle: Vec<String>) -> usize {

    let mut precedence:HashMap<&str,i32>=HashMap::new();
    precedence.insert("+",2);
    precedence.insert("*",1);
    run(puzzle,precedence)
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
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(puzzle.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}
