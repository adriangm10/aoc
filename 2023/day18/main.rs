use std::{io, fs};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/18_input.txt")?;
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("part1: {res1}, part2: {res2}");
    Ok(())
}

fn part1(input: &str) -> isize {
    let input: Vec<_> = input.lines().map(|line| {
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        (line[0], line[1].parse::<isize>().unwrap())
    }).collect();
    area(&input)
}

fn part2(input: &str) -> isize {
    let input: Vec<_> = input.lines().map(|line| {
        let hex = line.split_ascii_whitespace().last().unwrap().trim_start_matches("(").trim_end_matches(")");
        let dist = isize::from_str_radix(&hex[1..6], 16).unwrap();
        let dir = match &hex[6..7] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            d => unreachable!("Unknown dir {d}"),
        };
        (dir, dist)
    }).collect();
    area(&input)
}

fn area(input: &[(&str, isize)]) -> isize {
    let (perimeter, area, _) = input.iter().fold((0, 0, (0, 0)), |(p, a, (y, x)), (dir, dist)| {
        match *dir {
            "U" => (p + dist, a - x * dist, (y - dist, x)),
            "D" => (p + dist, a + x * dist, (y + dist, x)),
            "L" => (p + dist, a, (y, x - dist)),
            "R" => (p + dist, a, (y, x + dist)),
            _ => unreachable!("Unknown dir: {dir}")
        }
    });

    area + (perimeter / 2) + 1
}
