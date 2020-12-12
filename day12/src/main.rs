use std::fs;
use std::time::Instant;
use crate::Cardinal::{EAST, NORTH, SOUTH, WEST};
use std::ops::Neg;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i32)]
enum Cardinal {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

enum Direction {}


fn move_stuff(mut x: i32, mut y: i32, dir: Cardinal, step: i32) -> (i32, i32) {
    match dir {
        Cardinal::NORTH => {
            y += step;
        }
        Cardinal::EAST => {
            x += step;
        }
        Cardinal::SOUTH => {
            y -= step;
        }
        Cardinal::WEST => {
            x -= step;
        }
    }
    (x, y)
}

fn convert_to_enum(c: i32) -> Cardinal {
    match c {
        0 => NORTH,
        1 => EAST,
        2 => SOUTH,
        3 => WEST,
        _ => panic!("ERROR")
    }
}

fn rotate(rot: Cardinal, number: i32) -> Cardinal {
    let clock = ((((number / 90) + (rot as i32)) % 4) + 4) % 4;
    if clock != 1 && clock != 0 && clock != 2 && clock != 3 {
        panic!("EROOR")
    }
    convert_to_enum(clock)
}


fn translate(x: i32, y: i32, number: i32) -> (i32, i32) {
    let clock = (((number / 90) % 4) + 4) % 4;
    match clock {
        0 => (x, y),
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => unimplemented!("NOT POSSIBLE")
    }
}


fn part1(mut directions: Vec<String>) -> usize {
    let mut current_rotation = EAST;
    let mut x = 0;
    let mut y = 0;
    for direction in directions {
        let mut it = direction.chars();
        let command = it.next().unwrap();
        let number = it.collect::<String>().parse::<i32>().unwrap();
        match command {
            'N' => {
                let res = move_stuff(x, y, NORTH, number);
                x = res.0;
                y = res.1;
            }
            'E' => {
                let res = move_stuff(x, y, EAST, number);
                x = res.0;
                y = res.1;
            }
            'S' => {
                let res = move_stuff(x, y, SOUTH, number);
                x = res.0;
                y = res.1;
            }
            'W' => {
                let res = move_stuff(x, y, WEST, number);
                x = res.0;
                y = res.1;
            }
            'F' => {
                let res = move_stuff(x, y, current_rotation, number);
                x = res.0;
                y = res.1;
            }
            'R' => { current_rotation = rotate(current_rotation, number) }
            'L' => { current_rotation = rotate(current_rotation, number.neg()) }
            _ => { unimplemented!("not a pattern") }
        }
    }
    (x.abs() + y.abs()) as usize
}

fn part2(directions: Vec<String>) -> usize {
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    let mut ship_x: i64 = 0;
    let mut ship_y: i64 = 0;
    for direction in directions {
        let mut it = direction.chars();
        let command = it.next().unwrap();
        let number = it.collect::<String>().parse::<i32>().unwrap();
        match command {
            'N' => {
                let res = move_stuff(waypoint_x, waypoint_y, NORTH, number);
                waypoint_x = res.0;
                waypoint_y = res.1;
            }
            'E' => {
                let res = move_stuff(waypoint_x, waypoint_y, EAST, number);
                waypoint_x = res.0;
                waypoint_y = res.1;
            }
            'S' => {
                let res = move_stuff(waypoint_x, waypoint_y, SOUTH, number);
                waypoint_x = res.0;
                waypoint_y = res.1;
            }
            'W' => {
                let res = move_stuff(waypoint_x, waypoint_y, WEST, number);
                waypoint_x = res.0;
                waypoint_y = res.1;
            }
            'F' => {
                ship_x+=waypoint_x as i64 *number as i64;
                ship_y+=waypoint_y as i64 *number as i64;
            }
            'R' => {
                let res = translate(waypoint_x, waypoint_y, number);
                waypoint_x = res.0;
                waypoint_y = res.1;
            }
            'L' => {
                let res = translate(waypoint_x, waypoint_y, number.neg());
                waypoint_x = res.0;
                waypoint_y = res.1;
            }
            _ => { unimplemented!("not a pattern") }
        }
    }
    dbg!(waypoint_x,waypoint_y);
    (ship_x.abs() + ship_y.abs()) as usize
}

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut adapters: Vec<String> = vec![];
    for line in lines {
        adapters.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("Running part1");
    let now = Instant::now();
    println!("Found {}", part1(adapters.clone()));
    println!("Took {}us", now.elapsed().as_micros());

    println!("Running part2");
    let now = Instant::now();
    println!("Found {}", part2(adapters.clone()));
    println!("Took {}us", now.elapsed().as_micros());
}