use std::time::Instant;
use std::fs;
use std::cmp::max;

pub fn get_sids(rows: Vec<String>)->Vec<u16>{
    let mut sids:Vec<u16>=vec![];
    for row in rows {
        let r: String = row.get(0..7).unwrap().chars().map(|x| match x
        {
            'B' => '1',
            'F' => '0',
            _ => panic!("Not expecting that char")
        }).collect();

        let c: String = row.get(7..10).unwrap().chars().map(|x| match x
        {
            'R' => '1',
            'L' => '0',
            _ => panic!("Not expecting that char")
        }).collect();
        let row_number = u8::from_str_radix(&*r, 2).unwrap();
        let col_number = u8::from_str_radix(&*c, 2).unwrap();
        let sid = (row_number as u16) * 8 + (col_number as u16);
        sids.push(sid);
    }
    sids
}
fn part1(rows: Vec<String>) -> u16 {
    *get_sids(rows).iter().max().expect("at least one element")
}

pub fn part2(rows: Vec<String>) -> u16 {
    let sids=get_sids(rows);
    for i in 1..1023 {
        if !sids.contains(&(i as u16)) && sids.contains(&(i+1 as u16)) && sids.contains(&(i-1 as u16)){
           return i;
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut rows: Vec<String> = vec![];
    for line in lines {
        rows.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(rows.clone()));
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(rows.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}

