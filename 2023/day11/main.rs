use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/11_input.txt")?;
    let universe: Vec<_> = input.lines().collect();
    let expanded_universe = expand(universe);
    //println!("{:#?}", expanded_universe);
    let mut expansionsi = 0;
    let galaxies: Vec<_> = expanded_universe
        .iter()
        .enumerate()
        .map(move |(i, line)| {
            let mut expansionsj = 0;

            if line.chars().all(|c| c == 'M') {
                expansionsi += 1;
            }

            line.char_indices()
                .filter_map(move |(j, c)| {
                    if c == 'M' {
                        expansionsj += 1;
                    }
                    return if c == '#' {
                        //println!("inserted galaxie: {}, {}", i + expansionsi, j + expansionsj);
                        Some((i + expansionsi * 999999, j + expansionsj * 999999))
                    } else {
                        None
                    };
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += manhattan_distance(galaxies[i], galaxies[j]);
        }
    }
    println!("{sum}");
    Ok(())
}

fn expand(universe: Vec<&str>) -> Vec<String> {
    let mut expanded = vec![];

    for i in &universe {
        if i.chars().all(|c| c == '.') {
            expanded.push("M".repeat(universe.len()));
        } else {
            expanded.push(i.to_string());
        }
    }

    for i in 0..universe[0].len() {
        if universe.iter().all(|line| &line[i..i + 1] == ".") {
            expanded
                .iter_mut()
                .for_each(|line| line.replace_range(i..i + 1, "M"));
        }
    }

    return expanded;
}

fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    return p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
}
