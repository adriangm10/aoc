use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/4_input.txt")?;
    println!("winnings: {}, copies: {}", part_1(&input), part_2(&input));
    Ok(())
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut total = 0;
            if let Some((_, nums)) = line.split_once(": ") {
                if let Some((winning_nums, nums)) = nums.split_once(" | ") {
                    let winning_nums: HashSet<_> = winning_nums
                        .split(" ")
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect();

                    let nums: Vec<_> = nums
                        .split(" ")
                        .filter_map(|x| x.parse::<usize>().ok())
                        .filter(|x| winning_nums.contains(x))
                        .collect();

                    if nums.len() > 0 {
                        total = 2_usize.pow((nums.len() - 1) as u32);
                    }
                }
            }
            return total;
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut copies = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        if let Some((_, nums)) = line.split_once(": ") {
            if let Some((winning_nums, nums)) = nums.split_once(" | ") {
                let winning_nums: HashSet<_> = winning_nums
                    .split(" ")
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();

                let nums: Vec<_> = nums
                    .split(" ")
                    .filter_map(|x| x.parse::<usize>().ok())
                    .filter(|x| winning_nums.contains(x))
                    .collect();

                // println!("line: {i}, total matches: {}", nums.len());
                for num in 1..nums.len() + 1 {
                    copies[i + num] += copies[i];
                }
            }
        }
    }

    //println!("{:?}", copies);
    copies.iter().sum()
}
