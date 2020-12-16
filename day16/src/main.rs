use std::fs;
use std::time::Instant;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq,Clone,Hash)]
struct Contraint {
    first_range: (u16, u16),
    second_range: (u16, u16),
    name:String,
}

fn parse_constraint(puzzle: Vec<String>) -> Vec<Contraint> {
    let mut constraints = vec![];
    for str in puzzle {
        if str.is_empty() {
            break;

        }
        let mut it = str.split(": ");
        let _name = it.next().unwrap();
        let fields = it.next().unwrap();
        assert!(it.next().is_none());
        let mut it = fields.split(" or ");
        let field1 = it.next().unwrap();
        let field2 = it.next().unwrap();
        assert!(it.next().is_none());
        let mut it = field1.split("-");
        let mut it2 = field2.split("-");
        let constraint = Contraint {
            first_range: (it.next().unwrap().parse::<u16>().unwrap(), it.next().unwrap().parse::<u16>().unwrap()),
            second_range: (it2.next().unwrap().parse::<u16>().unwrap(), it2.next().unwrap().parse::<u16>().unwrap()),
            name: _name.to_string()
        };
        constraints.push(constraint);
    }
    constraints
}

fn parse_ticket(puzzle: Vec<String>, mut skip:u32) -> Vec<Vec<u16>> {
    let mut it =puzzle.iter();
    loop{
        if skip==0{
            break;
        }
        let res=it.next();
        if res.is_none(){
            break;
        }
        if res.unwrap().is_empty(){
            skip-=1;
        }

    }
    it.next();
    let mut result =vec![];
    loop {
        let res = it.next();
        if res.is_none() {
            break;
        }
        let line=res.unwrap();
        if line.is_empty(){
            break;
        }
        result.push(line.split(",").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>());
    }
    result
}



fn part1(puzzle: Vec<String>) -> usize {
    let constraint=parse_constraint(puzzle.clone());
    let _mine=parse_ticket(puzzle.clone(),1);
    let others=parse_ticket(puzzle.clone(),2);
    let mut eer =0usize;
    for ticket in others{
        for field in ticket {
            let mut good =false;
            for c in &constraint{
                if !( (field>c.first_range.1 || field<c.first_range.0) &&(field>c.second_range.1 || field<c.second_range.0) ) {
                    good=true;
                }
            }
            if !good{
                eer+=field as usize;
            }

        }
    }
    eer
}

fn filter(others:Vec<Vec<u16>>, constraint:Vec<Contraint>) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let (mut b, mut g)=(vec![], vec![]);
    for ticket in others{
        let mut bad=false;
        for field in &ticket {
            let mut good =false;
            for c in &constraint{
                if !( (field> &c.first_range.1 || field< &c.first_range.0) &&(field> &c.second_range.1 || field< &c.second_range.0) ) {
                    good=true;
                }
            }
            if !good{
                bad=true;
                break;
            }

        }
        if bad{
            b.push(ticket.clone());
        }else{
            g.push(ticket.clone());
        }
    }
    (b,g)
}

fn part2(puzzle: Vec<String>) -> usize {
    let constraint=parse_constraint(puzzle.clone());
    let mine=parse_ticket(puzzle.clone(),1);
    let others=parse_ticket(puzzle.clone(),2);
    let (_,good_tickets)=filter(others,constraint.clone());
    assert!(good_tickets.len()>0);
    let mut attributed_constraints =vec![];
    for i in 0..good_tickets.get(0).unwrap().len(){
        let mut good_contraints =vec![];
        for c in &constraint{
            let mut good =true;
            for el in &good_tickets{
                let field=el.get(i).unwrap();
                if (field> &c.first_range.1 || field< &c.first_range.0) &&(field> &c.second_range.1 || field< &c.second_range.0){
                    good=false;
                    break;
                }
            }
            if good{
                good_contraints.push((c,i));
            }

        }
        attributed_constraints.push(good_contraints);
    }
    attributed_constraints.sort_by(|a,b| a.len().cmp(&b.len()));
    let mut blacklist:Vec<&Contraint>=vec![];
    let mut map:HashMap<&Contraint,usize>=HashMap::new();
    for attributed_constraint in attributed_constraints {

        let rest=attributed_constraint.iter().filter(|&x| !blacklist.contains(&x.0)).collect::<Vec<&(&Contraint, usize)>>();
        assert_eq!(rest.len(), 1);
        let t=rest.first().unwrap();
        blacklist.push(t.0);
        map.insert(t.0, t.1);
    }
    let mut s =1;
    let my_ticket=mine.first().unwrap();
    for v in map.keys(){
        if v.name.starts_with("departure"){
            let pos=map.get(v).unwrap();
            let nbr=my_ticket.get(*pos).unwrap();
            s*= *nbr as usize;
        }
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
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(puzzle.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}