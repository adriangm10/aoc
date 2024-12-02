mod hand;

use std::{io, fs};
use hand::*;


fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/7_input.txt")?;
    let mut hands = parse_hands(&input);
    hands.sort();
    let res = hands.iter().enumerate().fold(0, |acc, (i, hand)|{
        acc + hand.bid * (i as u32 + 1)
    });
    // println!("{:#?}", hands);
    println!("{:?}", res);
    Ok(())
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input.lines().filter_map(|line| {
        Hand::try_from(line).ok()
    }).collect()
}
