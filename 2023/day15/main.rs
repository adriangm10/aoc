use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/15_input.txt")?;
    let p1: usize = part1(&input);
    println!("part1: {p1}");
    let p2: usize = part2(&input);
    println!("part1: {p2}");
    Ok(())
}

fn part1(input: &str) -> usize {
    input.trim_end().split(",").map(|x| hash(x)).sum()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

fn part2(input: &str) -> usize {
    const BOX_LIST: Vec<(&str, usize)> = Vec::new();
    let mut boxes = [BOX_LIST; 256];

    for x in input.trim_end().split(",") {
        if x.contains("=") {
            let (label, fl) = x.split_once("=").unwrap();
            let h = hash(label);
            if let Some(ind) = boxes[h].iter().position(|(lbl, _)| *lbl == label) {
                boxes[h].get_mut(ind).unwrap().1 = fl.parse().unwrap();
            } else {
                boxes[h].push((label, fl.parse().unwrap()));
            }
        } else {
            let label = x.trim_end_matches("-");
            let h = hash(label);
            if let Some(ind) = boxes[h].iter().position(|(lbl, _)| *lbl == label) {
                boxes[h].remove(ind);
            }
        }
    }

    //println!("{:?}", boxes);

    boxes.iter().enumerate().map(|(i, b)|{
        b.iter().enumerate().map(|(j, x)| {
            (i + 1) * (j + 1) * x.1
        }).sum::<usize>()
    }).sum()
}
