use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/2_input.txt")?;

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
    Ok(())
}

fn part_1(input: &str) -> usize {
    let cubes: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .filter_map(|line| {
            let (id, sets) = line.split_once(": ").unwrap();
            let mut sets = sets.split("; ");

            let impossible_set = sets.find(|set| {
                let set = set.split(", ");
                let mut set = set.filter(|cube| {
                    if let Some((n, color)) = cube.split_once(" ") {
                        if n.parse::<usize>().unwrap() > *cubes.get(color).unwrap() {
                            return true;
                        }
                    }
                    false
                });
                set.next().is_some()
            });

            if None == impossible_set {
                return Some(
                    id.split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                );
            }
            None
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, sets) = line.split_once(": ").unwrap();
            let sets = sets.split("; ");
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for set in sets {
                for cube in set.split(", ") {
                    if let Some((n, color)) = cube.split_once(" ") {
                        let n = n.parse().unwrap();
                        match color {
                            "red" if n > red => red = n,
                            "green" if n > green => green = n,
                            "blue" if n > blue => blue = n,
                            _ => (),
                        };
                    }
                }
            }
            red * green * blue
        })
        .sum()
}
