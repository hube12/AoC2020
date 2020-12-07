use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
#[derive(Copy,Clone,Eq, PartialEq,Ord, PartialOrd,Hash)]
struct BetterString{
    s:&'static str
}


// fn parse_rules(rules:Vec<String>)->GraphMap<&'static str,usize, Directed>{
//     let mut graph:GraphMap<&str,usize, Directed>=GraphMap::new();
//     for rule in rules{
//         let mut it =rule.split(" bags contain ");
//         let key=it.next().expect("no key");
//         let value=it.next().expect("No value");
//         let mut it=value.split(", ");
//         while let Some(cur)=it.next(){
//             let cur=cur.trim_end_matches(".");
//             let cur=cur.trim_end_matches("s");
//             let cur=cur.trim_end_matches(" bag");
//             let mut it2 =cur.split(" ");
//             let number=it2.next().expect("no number?").parse::<usize>();
//             if number.is_ok(){
//                 let number=number.unwrap();
//                 let bag =it2.fold(String::new(), |a, b| a + b + " ");
//                 let bag=bag.trim_end_matches(" ");
//                 graph.add_edge(key.clone(), bag.clone(), number.clone());
//             }else{
//                 eprintln!("Discarding {} for {}",key,value)
//             }
//         }
//         assert!(it.next().is_none());
//     }
//     graph
// }
const SHINY_GOLD: BetterString =BetterString{
    s: "shiny gold"
};

fn part1(rules:Vec<String>) ->usize{

    let mut graph:GraphMap<BetterString,usize, Directed>=GraphMap::new();
    for rule in rules{
        let mut it =rule.split(" bags contain ");
        let key=it.next().expect("no key");
        let value=it.next().expect("No value");
        let mut it=value.split(", ");
        while let Some(cur)=it.next(){
            let cur=cur.trim_end_matches(".");
            let cur=cur.trim_end_matches("s");
            let cur=cur.trim_end_matches(" bag");
            let mut it2 =cur.split(" ");
            let number=it2.next().expect("no number?").parse::<usize>();
            if number.is_ok(){
                let number=number.unwrap();
                let bag =it2.fold(String::new(), |a, b| a + b + " ");
                let bag=bag.trim_end_matches(" ");
                let keyr=BetterString{
                    s: key.clone().to_string().to_owned(),
                };
                let bagr=BetterString{
                    s: bag.clone(),
                };

                graph.add_edge(keyr, bagr, number.clone());
            }else{
                eprintln!("Discarding {} for {}",key,value)
            }
        }
        assert!(it.next().is_none());
    }
    let mut count:usize=0;
    graph.contains_node(SHINY_GOLD);
    count
}
fn part2(rules:Vec<String>)->usize{

0
}

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut rules: Vec<String> = vec![];
    for line in lines {
        rules.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(rules.clone()));
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(rules.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}