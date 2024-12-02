#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

pub fn find_marker(marker: &mut Vec<u8>){
    for i in 0..marker.len()-1 {
        if marker[i] == marker[marker.len() - 1] {
            *marker = marker[i+1..marker.len()].to_vec();
            break;
        }
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("Couldn't open the file");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("Couldn't read the file");
    let input = input.as_bytes();
    let mut marker = Vec::<u8>::new();
    for i in 0..input.len() {
        marker.push(input[i]);
        find_marker(&mut marker);
        if marker.len() == 14 {
            println!("Se han recorrido {} caracteres", i+1);
            break;
        }
    }
}
