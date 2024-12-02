#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::ops::Add;

fn main() {
    let mut f = File::open("input.txt").expect("Couldn't open the file");
    let mut lines = String::new();
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut stack: Vec<&str> = Vec::new();
    dirs.insert("/".to_string(), 0);
    f.read_to_string(&mut lines).expect("Couldn't read the file");
    let lines: Vec<&str> = lines.lines().collect();
    for line in lines {
        let command: Vec<&str> = line.split(" ").collect();
        if command[0] == "$" {
            if command[1] == "cd" {
                if command[2] == ".." {
                    stack.pop();
                }else {
                    if command[2] == "/" {
                        stack.clear(); 
                    }
                    stack.push(command[2]);
                    // println!("insertar {} en la pila", command[2]);
                }
            }
        }else {
            match command[0].parse::<usize>() {
                Ok(s) => {
                    for i in 0..stack.len() {
                        if i != 0{
                            // println!("actualizar {} en el hashmap", stack[0..i+1].join("/"));
                            dirs.insert(stack[0..i+1].join("/"), *dirs.get(&stack[0..i+1].join("/")).unwrap() + s);
                        }else{
                            dirs.insert("/".to_string(), dirs.get("/").unwrap() + s);
                        }
                    }
                },
                Err(_e) => {
                    dirs.insert(stack[0..stack.len()].join("/").add(format!("/{}", command[1]).as_str()), 0);
                    // println!("insertar {} en el hashmap", stack[0..stack.len()].join("/").add(format!("/{}", command[1]).as_str()));
                },
            };
        }
    }
    let used_space = dirs.get("/").unwrap();
    let unused_space = 70_000_000 - used_space;
    let space_to_delete = 30_000_000 - unused_space;
    let mut min = &usize::MAX;
    for size in dirs.values() {
        if size >= &space_to_delete && size < min {
            min = size;
        }
    }
    println!("min space to remove = {}", min);
}
