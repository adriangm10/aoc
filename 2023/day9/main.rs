use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/9_input.txt")?;
    let sequences = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let res1: i32 = sequences.clone().map(|sequence| extrapolate1(&sequence)).sum();

    let res2: i32 = sequences.map(|sequence| extrapolate2(&sequence)).sum();

    println!("part1: {res1}, part2: {res2}");
    Ok(())
}

fn extrapolate1(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut next_seq = Vec::new();
    for i in 1..sequence.len() {
        next_seq.push(sequence[i] - sequence[i - 1]);
    }

    return sequence.last().unwrap() + extrapolate1(&next_seq);
}

fn extrapolate2(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut next_seq = Vec::new();
    for i in 1..sequence.len() {
        next_seq.push(sequence[i] - sequence[i - 1]);
    }

    return sequence.first().unwrap() - extrapolate2(&next_seq);
}
