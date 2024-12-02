use std::{collections::HashMap, fs, io};

use num::Integer;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/8_input.txt")?;
    let (steps, map) = parse_map(&input);

    let step_num1 = part1(steps, &map, "AAA");
    let step_num2 = part2(steps, &map);
    println!("{}, {}", step_num1, step_num2);

    Ok(())
}

fn part1(steps: &str, map: &HashMap<&str, (&str, &str)>, start: &str) -> usize {
    let mut elem = start;
    let mut step = 0;
    let mut step_num = 0;

    loop {
        if step == steps.len() {
            step = 0;
        }

        let nodes = map.get(elem).unwrap();
        elem = if &steps[step..step + 1] == "L" {
            nodes.0
        } else {
            nodes.1
        };
        step += 1;
        step_num += 1;

        if &elem[2..3] == "Z" {
            break;
        }
    }

    return step_num;
}

fn part2(steps: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    let elems = get_starting_nodes(map);

    elems
        .iter()
        .map(|elem| part1(steps, map, elem))
        .fold(1, |acc, n| acc.lcm(&n))
}

fn parse_map(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let steps = lines.next().unwrap();

    let map: HashMap<&str, (&str, &str)> = lines
        .filter_map(|line| {
            if let Some((node, elems)) = line.split_once(" = ") {
                let elems = elems
                    .trim_start_matches("(")
                    .trim_end_matches(")")
                    .split_once(", ")
                    .unwrap();
                return Some((node, elems));
            }
            None
        })
        .collect();

    return (steps, map);
}

fn get_starting_nodes<'a>(map: &HashMap<&'a str, (&'a str, &'a str)>) -> Vec<&'a str> {
    map.keys()
        .to_owned()
        .filter_map(|key| {
            if &key[2..3] == "A" {
                return Some(*key);
            }
            None
        })
        .collect()
}
