#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp::{max, min};
use std::env::args;

fn parse_coord(coord: &str) -> (usize, usize) {
    let mut xy = coord.split(",");
    (
        xy.next().unwrap().parse::<usize>().unwrap(),
        xy.next().unwrap().parse::<usize>().unwrap()
    )
}

fn load_cave() -> (HashSet<(usize, usize)>, usize) {
    let mut f = File::open(args().nth(1).expect("insert file name")).expect("couldn't open the file");
    let mut lines = String::new();
    f.read_to_string(&mut lines).unwrap();
    let lines = lines.lines();
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut maxy: usize = 0;
    for line in lines {
        let mut r = line.split(" -> ");
        let prev = r.next().unwrap();
        let mut previous = parse_coord(prev);
        if maxy < previous.1 { maxy = previous.1; }
        while let Some(current) = r.next() {
            let c = parse_coord(current);
            if c.1 > maxy {
                maxy = c.1;
            }
            if previous.1 == c.1{
                for i in min(c.0, previous.0)..=max(c.0, previous.0) {
                    rocks.insert((i, c.1));
                }
            }else{
                for i in min(c.1, previous.1)..=max(c.1, previous.1) {
                    rocks.insert((c.0, i));
                }
            }
            previous = c;
        }
    }
    (rocks, maxy)
}

fn main() {
    let (mut cave, maxy) = load_cave();
    let mut done = false;
    let mut total_sand: usize = 0;
    loop {
        let mut sand: (usize, usize) = (500, 0);
        loop {
            let down = (sand.0, sand.1 + 1);
            if down.1 > maxy + 2 {
                cave.insert(sand);
                break;
            }
            if !cave.contains(&down) {
                sand = down;
                continue;
            }
            let down_left = (down.0 - 1, down.1);
            if !cave.contains(&down_left) {
                sand = down_left;
                continue;
            }
            let down_right = (down.0 + 1, down.1);
            if !cave.contains(&down_right) {
                sand = down_right;
                continue;
            }
            cave.insert(sand);
            total_sand += 1;
            if sand == (500, 0) {
                done = true;
            }
            break;
        }
        if done {
            break;
        }
    }
    println!("{total_sand}");
}
