#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("Couldn't open te file");
    let mut lines = String::new();
    let mut ranges: Vec<&str>;
    let mut range1: Vec<&str>;
    let mut range2: Vec<&str>;
    let mut total: usize = 0;
    f.read_to_string(&mut lines).expect("Couldn't read the file");
    let lines: Vec<&str> = lines.lines().collect();
    for line in lines {
        ranges = line.split(",").collect();
        range1 = ranges[0].split("-").collect();
        range2 = ranges[1].split("-").collect();
        if range1.len() > 1 && range2.len() > 1{
            // parte1
            // match (
            //     range1[0].parse::<u32>().unwrap(),
            //     range1[1].parse::<u32>().unwrap(),
            //     range2[0].parse::<u32>().unwrap(),
            //     range2[1].parse::<u32>().unwrap()
            // ){
            //     (a, b, c, d) if a >= c && b <= d => total += 1,
            //     (a, b, c, d) if a <= c && b >= d => total += 1,
            //     (_, _, _, _) => total += 0,
            // };
            match (
                range1[0].parse::<u32>().unwrap(),
                range1[1].parse::<u32>().unwrap(),
                range2[0].parse::<u32>().unwrap(),
                range2[1].parse::<u32>().unwrap()
            ){
                (a, _b, c, d) if a >= c && a <= d => total += 1,
                (_a, b, c, d) if b >= c && b <= d => total += 1,
                (a, b, c, _d) if c >= a && c <= b => total += 1,
                (a, b, _c, d) if d >= a && d <= b => total += 1,
                (_, _, _, _) => total += 0,
            };
        }
    }
    println!("el total es {}", total);
}
