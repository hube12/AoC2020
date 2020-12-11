use std::cmp::max;
use std::fs;
use std::time::Instant;

fn first_automata(x: usize, y: usize, seats: &Vec<String>, len_x: usize, len_y: usize) -> char {
    let mut count = 0;
    for off_x in -1..=1isize {
        let part_x: isize = x as isize + off_x;
        for off_y in -1..=1isize {
            let part_y: isize = y as isize + off_y;
            if part_x >= 0 && part_x < len_x as isize && part_y >= 0 && part_y < len_y as isize {
                if !(off_x == off_y && off_y == 0) {
                    let c: char = seats.get(part_y as usize).unwrap().chars().nth(part_x as usize).unwrap();
                    if c == '#' {
                        count += 1;
                    }
                }
            }
        }
    }
    if count >= 4 {
        'L'
    } else { '#' }
}

fn add(x: isize, it: isize) -> isize {
    x + it
}

fn sub(x: isize, it: isize) -> isize {
    x - it
}

fn process_diag(x: usize, y: usize, seats: &Vec<String>, len_x: usize, len_y: usize, square: usize, f1: fn(isize, isize) -> isize, f2: fn(isize, isize) -> isize) -> i32 {
    let mut count = 0;
    for it in 1..square as isize {
        let off_x = f1(x as isize, it);
        let off_y = f2(y as isize, it);
        if off_x >= 0 && off_x < len_x as isize && off_y >= 0 && off_y < len_y as isize {
            let c: char = seats.get(off_y as usize).unwrap().chars().nth(off_x as usize).unwrap();
            if c == 'L' {
                break;
            }
            if c == '#' {
                count += 1;
                break;
            }
        }
    }
    count
}

fn second_automata(x: usize, y: usize, seats: &Vec<String>, len_x: usize, len_y: usize) -> i32 {
    let mut count = 0;
    if x > 0 {
        for off_x in (0..x).rev() {
            let c: char = seats.get(y as usize).unwrap().chars().nth(off_x as usize).unwrap();
            if c == 'L' {
                break;
            }
            if c == '#' {
                count += 1;
                break;
            }
        }
    }
    for off_x in x + 1..len_x {
        let c: char = seats.get(y as usize).unwrap().chars().nth(off_x as usize).unwrap();
        if c == 'L' {
            break;
        }
        if c == '#' {
            count += 1;
            break;
        }
    }
    if y > 0 {
        for off_y in (0..y).rev() {
            let c: char = seats.get(off_y as usize).unwrap().chars().nth(x as usize).unwrap();
            if c == 'L' {
                break;
            }
            if c == '#' {
                count += 1;
                break;
            }
        }
    }

    for off_y in y + 1..len_y {
        let c: char = seats.get(off_y as usize).unwrap().chars().nth(x as usize).unwrap();
        if c == 'L' {
            break;
        }
        if c == '#' {
            count += 1;
            break;
        }
    }
    let square = max(len_y, len_x);
    count += process_diag(x, y, seats, len_x, len_y, square, add, sub);
    count += process_diag(x, y, seats, len_x, len_y, square, add, add);
    count += process_diag(x, y, seats, len_x, len_y, square, sub, sub);
    count += process_diag(x, y, seats, len_x, len_y, square, sub, add);

    count
}


fn process_automata(x: usize, y: usize, seats: &Vec<String>, current: char, len_x: usize, len_y: usize, second: bool) -> char {
    match current {
        'L' => {
            if !second {
                let mut b = true;
                for off_x in -1..=1isize {
                    let part_x: isize = x as isize + off_x;
                    for off_y in -1..=1isize {
                        let part_y: isize = y as isize + off_y;
                        if part_x >= 0 && part_x < len_x as isize && part_y >= 0 && part_y < len_y as isize {
                            if !(off_x == off_y && off_y == 0) {
                                let c: char = seats.get(part_y as usize).unwrap().chars().nth(part_x as usize).unwrap();
                                if c == '#' {
                                    b = false;
                                }
                            }
                        }
                    }
                }
                if b {
                    '#'
                } else { current }
            } else {
                if second_automata(x, y, seats, len_x, len_y) > 0 { 'L' } else { '#' }
            }
        }
        '#' => {
            if !second {
                first_automata(x, y, seats, len_x, len_y)
            } else {
                if second_automata(x, y, seats, len_x, len_y) >= 5 { 'L' } else { '#' }
            }
        }
        _ => { current }
    }
}

fn process_logic(mut seats: Vec<String>, second: bool) -> usize {
    let len_x = seats.get(0).unwrap().len();
    let len_y = seats.len();
    loop {
        let mut clone = seats.clone();
        for y in 0..len_y {
            let mut temp = String::with_capacity(len_x);
            for x in 0..len_x {
                let cur = seats.get(y).unwrap().chars().nth(x).unwrap();
                let char = process_automata(x, y, &seats, cur, len_x, len_y, second);
                temp.push(char);
            }
            clone[y] = temp;
        }
        let matching = seats.iter().zip(&clone).filter(|&(a, b)| a == b).count();
        if matching == len_y {
            break;
        }
        seats = clone.clone();
    }
    let mut s = 0;
    for i in 0..len_y {
        s += seats.get(i).unwrap().chars().filter(|&a| a == '#').count();
    }

    s
}

fn part1(seats: Vec<String>) -> usize {
    process_logic(seats, false)
}

fn part2(seats: Vec<String>) -> usize {
    process_logic(seats, true)
}

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut seats: Vec<String> = vec![];
    for line in lines {
        seats.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(seats.clone()));
    println!("Took {}ms", now.elapsed().as_millis());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(seats.clone()));
    println!("Took {}ms", now.elapsed().as_millis());
}