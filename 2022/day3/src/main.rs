#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

fn main() {
    static ALPHABET: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X',
        'Y', 'Z',
    ];
    let mut f = File::open("input.txt").expect("Couldn't open the file");
    let mut list = String::new();
    f.read_to_string(&mut list).expect("Couldn't read the file");
    let rucksacks: Vec<&str> = list.lines().collect();
    // let mut firstcomp: &str;
    // let mut seccomp: &str;
    let mut res = String::new();
    for i in (0..rucksacks.len()).step_by(3) {
        // parte 1 comparar la primera con la segunda y encontrar caracter repetido
        // firstcomp = i.get(..i.len()/2).expect("Algo ha fallado al conseguir el primer comp");
        // seccomp = i.get(i.len()/2..i.len()).expect("Algo ha fallado al conseguir el segundo comp");
        // parte2 comparar lo que llevan los grupos de 3 elfos y encontrar caracter repetido en los 3
        let first = rucksacks[i];
        let second = rucksacks[i + 1];
        let third = rucksacks[i + 2];
        let mut j = 0;
        let mut encontrado: bool = false;
        while !encontrado {
            for k in second.chars() {
                if k == first.chars().nth(j).unwrap() {
                    for l in third.chars() {
                        if l == k {
                            res.push(l);
                            encontrado = true;
                            break;
                        }
                    }
                    break;
                }
            }
            j += 1;
        }
    }
    let mut total: usize = 0;
    for i in res.chars() {
        for j in 0..52 {
            if i == ALPHABET[j] { 
                total += j+1;
                break;
            }
        }
    }
    println!("{}", total);
}
