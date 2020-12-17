use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use core::ops;
use std::hash::Hash;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct P1 {
    x: isize,
    y: isize,
    z: isize,
}

const P1_NULL: P1 = P1 {
    x: 0,
    y: 0,
    z: 0,
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct P2 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

const P2_NULL: P2 = P2 {
    x: 0,
    y: 0,
    z: 0,
    w: 0,
};

impl ops::Add<P1> for P1 {
    type Output = P1;

    fn add(self, rhs: P1) -> Self::Output {
        P1 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<P2> for P2 {
    type Output = P2;

    fn add(self, rhs: P2) -> Self::Output {
        P2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

type SparseGrid<T> = HashSet<T>;

type StepGrid<T> = HashMap<T, (bool, u32)>;

fn process_input(puzzle: Vec<String>) -> SparseGrid<P1> {
    let mut grid = HashSet::new();
    for (y, line) in puzzle.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(P1 { x: x as isize, y: y as isize, z: 0 });
            }
        }
    }
    grid
}

fn process_input_2(puzzle: Vec<String>) -> SparseGrid<P2> {
    let mut grid = HashSet::new();
    for (y, line) in puzzle.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(P2 { x: x as isize, y: y as isize, z: 0, w: 0 });
            }
        }
    }
    grid
}


fn get_directions3d() -> Vec<P1> {
    let mut directions = vec![];
    for i in -1isize..=1isize {
        for j in -1isize..=1isize {
            for k in -1isize..=1isize {
                let p: P1 = P1 {
                    x: i,
                    y: j,
                    z: k,
                };
                if p != P1_NULL {
                    directions.push(p);
                }
            }
        }
    }
    directions
}

fn get_directions4d() -> Vec<P2> {
    let mut directions = vec![];
    for i in -1isize..=1isize {
        for j in -1isize..=1isize {
            for k in -1isize..=1isize {
                for l in -1isize..=1isize {
                    let p: P2 = P2 {
                        x: i,
                        y: j,
                        z: k,
                        w: l,
                    };

                    if p != P2_NULL {
                        directions.push(p);
                    }
                }
            }
        }
    }
    directions
}


fn step<T: std::ops::Add<Output=T> + Eq + Hash + Copy>(grid: SparseGrid<T>, directions: Vec<T>) -> StepGrid<T> {
    let mut step_grid: StepGrid<T> = HashMap::new();
    for p in grid.iter() {
        for direction in &directions {
            let p1 = *p + *direction;
            if !step_grid.contains_key(&p1) {
                step_grid.insert(p1, (grid.contains(&p1), 0));
            }
            step_grid.get_mut(&p1).unwrap().1 += 1;
        }
    }
    step_grid
}

fn reduce<T: std::ops::Add<Output=T> + Eq + Hash + Copy>(step_grid:StepGrid<T>)->SparseGrid<T>{
    let mut grid:SparseGrid<T>=HashSet::new();
    for (p,(active,neighbors)) in step_grid.iter(){
        if *active && (*neighbors == 2 || *neighbors == 3) {
            grid.insert(*p);
        }else if !*active && (*neighbors) == 3 {
            grid.insert(*p);
        }
    }
    grid
}

fn part1(puzzle: Vec<String>) -> usize {
    let dir3d = get_directions3d();
    let mut grid =process_input(puzzle);
    for _ in 0..6{
        grid=reduce(step(grid.clone(), dir3d.clone()));
    }
    grid.len()
}

fn part2(puzzle: Vec<String>) -> usize {
    let dir4d = get_directions4d();
    let mut grid =process_input_2(puzzle);
    for _ in 0..6{
        grid=reduce(step(grid.clone(), dir4d.clone()));
    }
    grid.len()
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
    println!("Took {}ms", now.elapsed().as_millis());
}