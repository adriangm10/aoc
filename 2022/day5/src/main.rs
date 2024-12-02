use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("Couldn't open the file");
    let mut lines = String::new();
    let mut instructions: Vec<&str>;
    let mut i = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    f.read_to_string(&mut lines).expect("Couldn't read the file");
    let lines: Vec<&str> = lines.lines().collect();
    //cargar las pilas
    while lines[i] != "" {
        stacks.push(Vec::new());
        for j in lines[i].chars() {
            stacks[i].push(j);
        }
        i += 1;
    }
    //procesar las instrucciones
    for j in lines[i+1..lines.len()].to_vec() {
        instructions = j.split(" ").collect();
        if instructions.len() > 0 {
            //hacer los movimientos de las instrucciones en la pila
            //Parte 1://
            // for _i in 0..instructions[1].parse::<usize>().unwrap() {
            //     match stacks[instructions[3].parse::<usize>().unwrap()-1].pop() {
            //         Some(c) => stacks[instructions[5].parse::<usize>().unwrap()-1].push(c),
            //         None => ()
            //     };
            // }
            //Parte2://
            let size = instructions[1].parse::<usize>().unwrap();
            let stack1 = stacks[instructions[3].parse::<usize>().unwrap().saturating_sub(1)].clone();
            let mut stack2 = stacks[instructions[5].parse::<usize>().unwrap().saturating_sub(1)].clone();
            stack2.append(&mut stack1[stack1.len().saturating_sub(size)..stack1.len()].to_vec());
            stacks[instructions[5].parse::<usize>().unwrap().saturating_sub(1)] = stack2;
            stacks[instructions[3].parse::<usize>().unwrap().saturating_sub(1)].resize(stack1.len()-size, ' ');
        }
    }
    //sacar la solucion
    let mut sol = String::new();
    for mut j in stacks {
        match j.pop() {
            Some(c) => sol.push(c),
            None => (),
        };
    }
    println!("{}", sol);
}
