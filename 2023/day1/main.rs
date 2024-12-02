use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/1_input.txt")?;
    let mut total = 0;
    let digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in input.lines() {
        let mut first = 0;
        let mut last = 0;


        for (i, char) in line.char_indices() {
            match char.to_digit(10) {
                Some(d) => {
                    first = if first == 0 { d } else { first };
                    last = d;
                }
                None => {
                    if let Some(text) = line.get(i..i + 3) {
                        if let Some(d) = digits.get(text) {
                            first = if first == 0 { *d } else { first };
                            last = *d;
                            continue;
                        }
                    } 
                    if let Some(text) = line.get(i..i + 4) {
                        if let Some(d) = digits.get(text) {
                            first = if first == 0 { *d } else { first };
                            last = *d;
                            continue;
                        }
                    } 
                    if let Some(text) = line.get(i..i + 5) {
                        if let Some(d) = digits.get(text) {
                            first = if first == 0 { *d } else { first };
                            last = *d;
                        }
                    }
                }
            };
        }

        total += first * 10 + last;
    }

    println!("{}", total);
    Ok(())
}
