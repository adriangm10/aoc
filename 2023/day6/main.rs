use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/6_input.txt")?;

    println!("part1: {:?}, part2: {:?}", part1(&input), part2(&input));

    Ok(())
}

fn get_possibilites((time, record): (usize, usize)) -> usize {
    // if (time - x) * x > record => x^2 - time*x + record < 0
    let root = ((time.pow(2) - 4 * record) as f64).sqrt();
    let a = ((time as f64 - root) / 2.0).floor() as usize;
    let b = ((time as f64 + root) / 2.0).ceil() as usize;
    return b - a - 1;
}

fn part1(input: &str) -> usize {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok());

    let records = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok());

    let res = times.zip(records).map(get_possibilites);

    res.fold(1, |mul, x| mul * x)
}

fn part2(input: &str) -> usize {
    let time = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .fold(0, |n, x| format!("{}{}", n, x).parse::<usize>().unwrap());

    let record = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .fold(0, |n, x| format!("{}{}", n, x).parse::<usize>().unwrap());

    return get_possibilites((time, record));
}
